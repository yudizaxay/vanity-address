//! OpenCL GPU grind (Solana + EVM) with mandatory CPU re-verify of any match.
//!
//! Enabled with the `opencl` feature. When the feature is off, or no device is
//! present, callers fall back to the rayon CPU grinder.

mod device;
mod engine;

#[cfg(feature = "opencl")]
mod opencl_backend;
#[cfg(feature = "opencl")]
mod pattern_pack;

pub use device::{gpu_available, list_gpu_devices, GpuDeviceInfo};
pub use engine::{benchmark_gpu, grind_gpu, GpuGrindReport, GpuMode};

/// Chains the OpenCL MVP can accelerate. Everything else stays on CPU.
pub fn gpu_supports_chain(chain_id: &str) -> bool {
    use crate::chain::ChainGrinder;
    match crate::chains::Chain::from_id(chain_id) {
        Ok(c) => c.id() == "evm" || c.id() == "sol",
        Err(_) => matches!(
            chain_id.to_ascii_lowercase().as_str(),
            "sol" | "solana" | "evm" | "eth" | "ethereum"
        ),
    }
}

/// Chains where `--gpu auto` picks the GPU. Solana's OpenCL kernel is still slower than the
/// CPU grinder (measured ~5x on Apple M4), so it only runs with `--gpu require`.
pub fn gpu_auto_chain(chain_id: &str) -> bool {
    use crate::chain::ChainGrinder;
    crate::chains::Chain::from_id(chain_id)
        .map(|c| c.id() == "evm")
        .unwrap_or(false)
}

/// Single place that decides whether a grind runs on the GPU, shared by CLI and desktop.
/// The GPU path finds one match at a time, so `count > 1` stays on the CPU.
pub fn gpu_should_use(mode: GpuMode, chain_id: &str, count: usize) -> bool {
    if count > 1 {
        return false;
    }
    match mode {
        GpuMode::Off => false,
        GpuMode::Auto => gpu_auto_chain(chain_id),
        GpuMode::Require => gpu_supports_chain(chain_id),
    }
}
