use crate::chain::{ChainGrinder, KeypairResult};
use crate::pattern::Pattern;
use crate::system::{build_thread_pool, SystemProfile};
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct GrindResult {
    pub keypair: KeypairResult,
    pub attempts: u64,
    pub elapsed_secs: f64,
}

/// Cooperative stop signal shared between a caller (UI/CLI) and a running grind.
#[derive(Clone, Default)]
pub struct CancelToken(Arc<AtomicBool>);

impl CancelToken {
    pub fn new() -> Self {
        Self(Arc::new(AtomicBool::new(false)))
    }

    pub fn cancel(&self) {
        self.0.store(true, Ordering::Relaxed);
    }

    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

/// Run a short warm-up grind to measure real keys/sec on this machine.
pub fn benchmark<G: ChainGrinder>(
    grinder: G,
    profile: &SystemProfile,
    duration_secs: f64,
) -> Result<f64, String> {
    let duration_secs = duration_secs.clamp(0.5, 10.0);
    let pool = build_thread_pool(profile)?;
    let counter = AtomicU64::new(0);
    let start = Instant::now();
    let deadline = start + Duration::from_secs_f64(duration_secs);

    pool.install(|| {
        (0..profile.worker_threads).into_par_iter().for_each(|_| {
            let mut local = 0u64;
            while Instant::now() < deadline {
                let _ = grinder.grind_attempt();
                local += 1;
            }
            counter.fetch_add(local, Ordering::Relaxed);
        });
    });

    let elapsed = start.elapsed().as_secs_f64().max(0.001);
    Ok(counter.load(Ordering::Relaxed) as f64 / elapsed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SolanaGrinder;

    #[test]
    fn benchmark_returns_positive_rate() {
        let grinder = SolanaGrinder;
        let profile = SystemProfile::detect().with_threads(2);
        let rate = benchmark(grinder, &profile, 0.5).expect("benchmark");
        assert!(rate > 1_000.0, "expected at least 1k keys/sec, got {rate}");
    }
}

pub fn grind<G: ChainGrinder>(
    grinder: G,
    pattern: Pattern,
    profile: &SystemProfile,
    cancel: &CancelToken,
    on_progress: impl Fn(u64, f64, f64) + Sync,
) -> Result<GrindResult, String> {
    grind_patterns(grinder, &[pattern], profile, cancel, on_progress)
}

/// Grind until any pattern in `patterns` matches (OR semantics).
pub fn grind_patterns<G: ChainGrinder>(
    grinder: G,
    patterns: &[Pattern],
    profile: &SystemProfile,
    cancel: &CancelToken,
    on_progress: impl Fn(u64, f64, f64) + Sync,
) -> Result<GrindResult, String> {
    if patterns.is_empty() {
        return Err("at least one pattern is required".into());
    }
    let expected = crate::pattern::expected_attempts_any(
        &patterns
            .iter()
            .map(|p| grinder.expected_attempts(p))
            .collect::<Vec<_>>(),
    );
    let counter = AtomicU64::new(0);
    let start = Instant::now();
    let progress_every = profile.progress_interval;

    let pool = build_thread_pool(profile)?;

    let outcome = pool.install(|| {
        rayon::iter::repeat(()).find_map_any(|_| {
            if cancel.is_cancelled() {
                return Some(None);
            }

            let n = counter.fetch_add(1, Ordering::Relaxed) + 1;
            if n.is_multiple_of(progress_every) {
                let secs = start.elapsed().as_secs_f64();
                let rate = n as f64 / secs;
                let eta_min = (expected - n as f64).max(0.0) / rate / 60.0;
                on_progress(n, rate, eta_min);
            }

            let (address, attempt) = grinder.grind_attempt();
            if patterns.iter().any(|p| grinder.matches(&address, p)) {
                Some(Some(grinder.finalize(attempt)))
            } else {
                None
            }
        })
    });

    let keypair = match outcome {
        Some(Some(kp)) => kp,
        Some(None) => return Err("cancelled".to_string()),
        None => return Err("grinding stopped before a match was found".to_string()),
    };

    let elapsed = start.elapsed();
    Ok(GrindResult {
        keypair,
        attempts: counter.load(Ordering::Relaxed),
        elapsed_secs: elapsed.as_secs_f64(),
    })
}

/// Find up to `count` matching keypairs (restarts grind after each hit).
pub fn grind_n<G: ChainGrinder + Clone>(
    grinder: G,
    patterns: &[Pattern],
    count: usize,
    profile: &SystemProfile,
    cancel: &CancelToken,
    on_progress: impl Fn(u64, f64, f64, usize) + Sync,
) -> Result<Vec<GrindResult>, String> {
    if count == 0 {
        return Err("--count must be at least 1".into());
    }
    let mut results = Vec::with_capacity(count);
    let mut total_attempts = 0u64;
    let wall_start = Instant::now();
    for i in 0..count {
        if cancel.is_cancelled() {
            break;
        }
        let found = grind_patterns(grinder.clone(), patterns, profile, cancel, |a, r, e| {
            on_progress(total_attempts + a, r, e, i + 1);
        })?;
        total_attempts += found.attempts;
        results.push(GrindResult {
            keypair: found.keypair,
            attempts: total_attempts,
            elapsed_secs: wall_start.elapsed().as_secs_f64(),
        });
    }
    if results.is_empty() {
        return Err("cancelled".into());
    }
    Ok(results)
}
