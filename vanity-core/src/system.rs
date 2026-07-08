use rayon::ThreadPoolBuilder;
use std::fmt;

/// Detected host capabilities used to tune the grinder before work starts.
#[derive(Debug, Clone)]
pub struct SystemProfile {
    pub logical_cpus: usize,
    pub physical_cpus: Option<usize>,
    pub total_memory_mb: u64,
    pub available_memory_mb: u64,
    pub worker_threads: usize,
    pub progress_interval: u64,
    pub memory_pressure: MemoryPressure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryPressure {
    Comfortable,
    Moderate,
    Low,
}

impl fmt::Display for MemoryPressure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryPressure::Comfortable => write!(f, "comfortable"),
            MemoryPressure::Moderate => write!(f, "moderate"),
            MemoryPressure::Low => write!(f, "low"),
        }
    }
}

impl SystemProfile {
    /// Probe the current machine and derive grinding settings.
    pub fn detect() -> Self {
        let logical_cpus = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        let (physical_cpus, total_memory_mb, available_memory_mb) = probe_sysinfo();

        let memory_pressure = classify_memory_pressure(available_memory_mb);
        let worker_threads =
            compute_worker_threads(logical_cpus, physical_cpus, available_memory_mb);
        let progress_interval = compute_progress_interval(worker_threads);

        Self {
            logical_cpus,
            physical_cpus,
            total_memory_mb,
            available_memory_mb,
            worker_threads,
            progress_interval,
            memory_pressure,
        }
    }

    /// Override thread count (e.g. from `--threads` CLI flag).
    pub fn with_threads(mut self, threads: usize) -> Self {
        let threads = threads.clamp(1, self.logical_cpus);
        self.worker_threads = threads;
        self.progress_interval = compute_progress_interval(threads);
        self
    }

    pub fn summary_line(&self) -> String {
        let cores = self.cpu_description();

        format!(
            "{cores} · {:.1} GB RAM ({:.1} GB free) · {} workers · memory: {}",
            self.total_memory_mb as f64 / 1024.0,
            self.available_memory_mb as f64 / 1024.0,
            self.worker_threads,
            self.memory_pressure,
        )
    }

    pub fn cpu_description(&self) -> String {
        match self.physical_cpus {
            Some(physical) if physical != self.logical_cpus => {
                format!("{} logical · {} physical", self.logical_cpus, physical)
            }
            Some(physical) => format!("{physical} cores"),
            None => format!("{} cores", self.logical_cpus),
        }
    }

    pub fn worker_description(&self) -> String {
        let reserved = self.logical_cpus.saturating_sub(self.worker_threads);
        if reserved > 0 {
            format!(
                "{} threads grinding · {} core{} reserved for OS",
                self.worker_threads,
                reserved,
                if reserved == 1 { "" } else { "s" }
            )
        } else {
            format!("{} threads (all cores)", self.worker_threads)
        }
    }

    pub fn estimated_keys_per_sec(&self, chain_id: &str) -> f64 {
        let per_thread = if chain_id == "evm" { 35_000.0 } else { 80_000.0 };
        per_thread * self.worker_threads as f64
    }
}

fn probe_sysinfo() -> (Option<usize>, u64, u64) {
    let mut sys = sysinfo::System::new();
    sys.refresh_memory();

    let total_memory_mb = sys.total_memory() / 1024 / 1024;
    let mut available_memory_mb = sys.available_memory() / 1024 / 1024;

    // macOS often reports near-zero "available" RAM — use a sane fallback.
    if available_memory_mb < 256 && total_memory_mb > 512 {
        available_memory_mb = (total_memory_mb * 3) / 4;
    }

    let physical_cpus = sys.physical_core_count();

    (physical_cpus, total_memory_mb, available_memory_mb)
}

fn classify_memory_pressure(available_mb: u64) -> MemoryPressure {
    if available_mb < 512 {
        MemoryPressure::Low
    } else if available_mb < 2_048 {
        MemoryPressure::Moderate
    } else {
        MemoryPressure::Comfortable
    }
}

/// How many rayon workers to spawn for this host.
pub fn compute_worker_threads(
    logical_cpus: usize,
    physical_cpus: Option<usize>,
    available_mem_mb: u64,
) -> usize {
    let logical_cpus = logical_cpus.max(1);

    // Leave one core for the OS / terminal on machines with headroom.
    let mut threads = match logical_cpus {
        1 => 1,
        2 => 2,
        n => n - 1,
    };

    // Prefer not to oversubscribe physical cores when SMT is present.
    if let Some(physical) = physical_cpus {
        if physical > 0 && threads > physical {
            threads = physical;
        }
    }

    // Back off when memory is tight — each worker holds crypto state on its stack.
    let mem_cap = match available_mem_mb {
        m if m < 512 => 1,
        m if m < 1_024 => 2,
        m if m < 2_048 => 4,
        m if m < 4_096 => 8,
        _ => usize::MAX,
    };

    threads.min(mem_cap).max(1)
}

/// Progress callback frequency — scales with worker count, bounded to limit overhead.
pub fn compute_progress_interval(worker_threads: usize) -> u64 {
    (50_000_u64 * worker_threads as u64).clamp(100_000, 2_000_000)
}

/// Build a rayon pool sized for this profile.
pub fn build_thread_pool(profile: &SystemProfile) -> Result<rayon::ThreadPool, String> {
    ThreadPoolBuilder::new()
        .num_threads(profile.worker_threads)
        .thread_name(|i| format!("vanity-worker-{i}"))
        .build()
        .map_err(|e| format!("Failed to configure thread pool: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_core_machine_uses_one_worker() {
        assert_eq!(compute_worker_threads(1, Some(1), 16_384), 1);
    }

    #[test]
    fn reserves_core_on_typical_laptop() {
        assert_eq!(compute_worker_threads(8, Some(8), 16_384), 7);
    }

    #[test]
    fn low_memory_caps_workers() {
        assert_eq!(compute_worker_threads(16, Some(16), 800), 2);
    }

    #[test]
    fn progress_interval_scales_with_threads() {
        assert_eq!(compute_progress_interval(1), 100_000);
        assert_eq!(compute_progress_interval(8), 400_000);
        assert_eq!(compute_progress_interval(64), 2_000_000);
    }
}
