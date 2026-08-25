pub mod chain;
pub mod chains;
pub mod estimate;
#[cfg(feature = "native")]
pub mod grinder;
pub mod pattern;
#[cfg(feature = "native")]
pub mod system;

pub use chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
#[cfg(not(target_arch = "wasm32"))]
pub use chains::SolanaGrinder;
pub use chains::{Chain, EvmGrinder, MENU_CHAINS};
pub use estimate::{
    effective_pattern_chars, format_attempts, format_duration, grind_estimate, GrindEstimate,
    PatternRisk,
};
#[cfg(feature = "native")]
pub use grinder::{benchmark, grind, CancelToken, GrindResult};
pub use pattern::Pattern;
#[cfg(feature = "native")]
pub use system::{build_thread_pool, MemoryPressure, SystemProfile};
