pub mod chain;
pub mod chains;
pub mod estimate;
#[cfg(feature = "native")]
pub mod grinder;
pub mod pattern;
#[cfg(feature = "native")]
pub mod system;
pub mod verify;

pub use chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
pub use chains::{Chain, Create2Grinder, EvmGrinder, SolanaGrinder, MENU_CHAINS};
pub use estimate::{
    default_keys_per_sec, effective_pattern_chars, format_attempts, format_duration,
    grind_estimate, GrindEstimate, PatternRisk,
};
#[cfg(feature = "native")]
pub use grinder::{benchmark, grind, grind_n, grind_patterns, CancelToken, GrindResult};
pub use pattern::{
    expand_pattern_alternatives, expected_attempts_any, matches_full, Pattern, PatternAlternative,
};
#[cfg(feature = "native")]
pub use system::{build_thread_pool, MemoryPressure, SystemProfile};
pub use verify::verify_address;

/// Expand comma-OR alternatives then `build_pattern` each (shared by CLI / WASM / desktop).
pub fn build_pattern_list(
    chain: &impl ChainGrinder,
    prefix: Option<&str>,
    suffix: Option<&str>,
    contains: Option<&str>,
    exact: bool,
) -> Result<Vec<Pattern>, String> {
    let alts = expand_pattern_alternatives(prefix, suffix, contains)?;
    let mut patterns = Vec::with_capacity(alts.len());
    for (p, s, c) in &alts {
        patterns.push(chain.build_pattern(p.as_deref(), s.as_deref(), c.as_deref(), exact)?);
    }
    if patterns.is_empty() {
        return Err("Provide at least one of --prefix, --suffix, or --contains".into());
    }
    Ok(patterns)
}

/// Description for one or many OR patterns.
pub fn patterns_description(patterns: &[Pattern]) -> String {
    if patterns.is_empty() {
        return String::new();
    }
    if patterns.len() == 1 {
        return patterns[0].description();
    }
    patterns
        .iter()
        .map(|p| p.description())
        .collect::<Vec<_>>()
        .join(" OR ")
}
