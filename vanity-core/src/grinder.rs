use crate::chain::{ChainGrinder, KeypairResult};
use crate::pattern::Pattern;
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
    progress_every: u64,
    on_progress: impl Fn(u64, f64, f64) + Sync,
) -> Option<GrindResult> {
    let expected = grinder.expected_attempts(&pattern);
    let counter = AtomicU64::new(0);
    let start = Instant::now();

    let keypair = rayon::iter::repeat(()).find_map_any(|_| {
        let n = counter.fetch_add(1, Ordering::Relaxed) + 1;
        if n.is_multiple_of(progress_every) {
            let secs = start.elapsed().as_secs_f64();
            let rate = n as f64 / secs;
            let eta_min = (expected - n as f64).max(0.0) / rate / 60.0;
            on_progress(n, rate, eta_min);
        }

        let keypair = grinder.generate_keypair();
        if grinder.matches(&keypair.address, &pattern) {
            Some(keypair)
        } else {
            None
        }
    })?;

    let elapsed = start.elapsed();
    Some(GrindResult {
        keypair,
        attempts: counter.load(Ordering::Relaxed),
        elapsed_secs: elapsed.as_secs_f64(),
    })
}
