use crate::chain::{ChainGrinder, KeypairResult};
use crate::pattern::Pattern;
use crate::system::{build_thread_pool, SystemProfile};
use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

pub struct GrindResult {
    pub keypair: KeypairResult,
    pub attempts: u64,
    pub elapsed_secs: f64,
}

pub fn grind<G: ChainGrinder>(
    grinder: G,
    pattern: Pattern,
    profile: &SystemProfile,
    on_progress: impl Fn(u64, f64, f64) + Sync,
) -> Result<GrindResult, String> {
    let expected = grinder.expected_attempts(&pattern);
    let counter = AtomicU64::new(0);
    let start = Instant::now();
    let progress_every = profile.progress_interval;

    let pool = build_thread_pool(profile)?;

    let keypair = pool
        .install(|| {
            rayon::iter::repeat(()).find_map_any(|_| {
                let n = counter.fetch_add(1, Ordering::Relaxed) + 1;
                if n.is_multiple_of(progress_every) {
                    let secs = start.elapsed().as_secs_f64();
                    let rate = n as f64 / secs;
                    let eta_min = (expected - n as f64).max(0.0) / rate / 60.0;
                    on_progress(n, rate, eta_min);
                }

                let (address, attempt) = grinder.grind_attempt();
                if grinder.matches(&address, &pattern) {
                    Some(grinder.finalize(attempt))
                } else {
                    None
                }
            })
        })
        .ok_or_else(|| "grinding stopped before a match was found".to_string())?;

    let elapsed = start.elapsed();
    Ok(GrindResult {
        keypair,
        attempts: counter.load(Ordering::Relaxed),
        elapsed_secs: elapsed.as_secs_f64(),
    })
}
