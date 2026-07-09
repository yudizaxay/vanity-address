use crate::state::AppState;
use serde::Serialize;
use std::io::Write;
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_dialog::DialogExt;
use vanity_core::{
    benchmark, grind, grind_estimate, CancelToken, Chain, ChainGrinder, GrindResult, Pattern,
    PatternRisk, SystemProfile, MENU_CHAINS,
};

const BENCHMARK_SECS: f64 = 2.0;

#[derive(Serialize)]
pub struct ChainInfo {
    id: String,
    display_name: String,
    menu_label: String,
    supports_exact_case: bool,
    pattern_hint: String,
}

#[tauri::command]
pub fn list_chains() -> Vec<ChainInfo> {
    MENU_CHAINS
        .iter()
        .enumerate()
        .filter_map(|(i, (id, menu_label))| {
            let chain = Chain::from_menu_index(i)?;
            Some(ChainInfo {
                id: id.to_string(),
                display_name: chain.display_name().to_string(),
                menu_label: menu_label.to_string(),
                supports_exact_case: chain.supports_exact_case(),
                pattern_hint: chain.pattern_hint().to_string(),
            })
        })
        .collect()
}

#[derive(Serialize)]
pub struct SystemInfo {
    cpu_description: String,
    worker_description: String,
    total_memory_gb: f64,
    available_memory_gb: f64,
    memory_pressure: String,
    summary_line: String,
    estimated_keys_per_sec: f64,
}

#[tauri::command]
pub fn get_system_profile(chain: String) -> Result<SystemInfo, String> {
    let chain = Chain::from_id(&chain)?;
    let profile = SystemProfile::detect();
    Ok(SystemInfo {
        cpu_description: profile.cpu_description(),
        worker_description: profile.worker_description(),
        total_memory_gb: profile.total_memory_mb as f64 / 1024.0,
        available_memory_gb: profile.available_memory_mb as f64 / 1024.0,
        memory_pressure: profile.memory_pressure.to_string(),
        summary_line: profile.summary_line(),
        estimated_keys_per_sec: profile.estimated_keys_per_sec(chain.id()),
    })
}

#[derive(Serialize, Debug)]
pub struct EstimateResult {
    pattern_description: String,
    case_mode: String,
    attempts_label: String,
    time_label: String,
    difficulty: String,
    difficulty_bars: String,
    risk: String,
    pattern_chars: usize,
    warning: Option<String>,
    length_guide: Option<String>,
    prefix_match: String,
    suffix_match: String,
    ignore_case: bool,
}

fn build_pattern_checked(chain: &Chain, prefix: &str, suffix: &str, exact: bool) -> Result<Pattern, String> {
    if exact && !chain.supports_exact_case() {
        return Err(format!(
            "Exact case is not supported for {}",
            chain.display_name()
        ));
    }
    let prefix = if prefix.is_empty() { None } else { Some(prefix) };
    let suffix = if suffix.is_empty() { None } else { Some(suffix) };
    if prefix.is_none() && suffix.is_none() {
        return Err("Enter a prefix and/or suffix".to_string());
    }
    chain.build_pattern(prefix, suffix, exact)
}

fn risk_label(risk: PatternRisk) -> &'static str {
    match risk {
        PatternRisk::None => "None",
        PatternRisk::Caution => "Caution",
        PatternRisk::Long => "Long",
        PatternRisk::Impractical => "Impractical",
    }
}

fn risk_warning(estimate: &vanity_core::GrindEstimate) -> Option<String> {
    match estimate.risk {
        PatternRisk::None => None,
        PatternRisk::Caution => Some(
            "This pattern may take hours or longer. 4–6 characters is usually realistic.".to_string(),
        ),
        PatternRisk::Long => Some(
            "Long pattern — weeks or months on this machine. Strongly consider shortening it."
                .to_string(),
        ),
        PatternRisk::Impractical => Some(format!(
            "{} characters is NOT practical on a single PC (years to centuries+). Vanity grinds are probabilistic — you will almost certainly never find a match. Recommended: 2–4 chars · 6 max for patient grinds.",
            estimate.pattern_chars
        )),
    }
}

fn length_guide(chain_id: &str, len: usize) -> String {
    let hex_chains = ["evm", "aptos", "sui", "near"];
    let length_hint = match len {
        0..=3 => "Great length — usually seconds to minutes",
        4..=5 => "OK — minutes to ~1 hour",
        6..=7 => "Getting long — hours to days",
        8..=9 => "Very long — days to weeks+",
        _ => "Too long for a single machine — use ≤6 chars",
    };
    let rule = if hex_chains.contains(&chain_id) {
        "Rule of thumb: 2→sec · 4→min · 6→~30min · 8+→hours+"
    } else {
        "Rule of thumb: 2→sec · 4→min · 6→~1hr · 8+→days+"
    };
    format!("{len}-char pattern: {length_hint}. {rule}")
}

#[tauri::command]
pub fn estimate(
    chain: String,
    prefix: String,
    suffix: String,
    exact: bool,
) -> Result<EstimateResult, String> {
    let chain = Chain::from_id(&chain)?;
    let pattern = build_pattern_checked(&chain, &prefix, &suffix, exact)?;
    let expected = chain.expected_attempts(&pattern);
    let profile = SystemProfile::detect();
    let est = grind_estimate(expected, profile.estimated_keys_per_sec(chain.id()), &pattern);

    let warning = risk_warning(&est);
    let length_guide = if pattern.has_prefix() || pattern.has_suffix() {
        Some(length_guide(chain.id(), est.pattern_chars))
    } else {
        None
    };

    Ok(EstimateResult {
        pattern_description: pattern.description(),
        case_mode: pattern.case_mode().to_string(),
        attempts_label: est.attempts_label,
        time_label: est.time_label,
        difficulty: est.difficulty.to_string(),
        difficulty_bars: est.difficulty_bars,
        risk: risk_label(est.risk).to_string(),
        pattern_chars: est.pattern_chars,
        warning,
        length_guide,
        prefix_match: pattern.prefix_match.clone(),
        suffix_match: pattern.suffix_match.clone(),
        ignore_case: pattern.ignore_case,
    })
}

#[derive(Serialize, Clone)]
struct ProgressPayload {
    attempts: u64,
    rate: f64,
    eta_min: f64,
}

#[derive(Serialize, Clone)]
struct SpeedPayload {
    rate: f64,
    time_label: String,
}

#[derive(Serialize, Clone)]
struct KeyExportPayload {
    label: String,
    value: String,
    hint: Option<String>,
}

#[derive(Serialize, Clone)]
struct DonePayload {
    address: String,
    exports: Vec<KeyExportPayload>,
    attempts: u64,
    elapsed_secs: f64,
    chain_display_name: String,
    pattern_description: String,
    case_mode: String,
    prefix_match: String,
    suffix_match: String,
    ignore_case: bool,
}

#[derive(Serialize, Clone)]
struct ErrorPayload {
    message: String,
}

#[tauri::command]
pub fn start_grind(
    app: AppHandle,
    state: State<AppState>,
    chain: String,
    prefix: String,
    suffix: String,
    exact: bool,
    force: bool,
) -> Result<(), String> {
    {
        let mut job = state.job.lock().map_err(|_| "internal lock error".to_string())?;
        if job.is_some() {
            return Err("A grind is already running".to_string());
        }
        let chain_parsed = Chain::from_id(&chain)?;
        let pattern = build_pattern_checked(&chain_parsed, &prefix, &suffix, exact)?;
        let expected = chain_parsed.expected_attempts(&pattern);
        let profile = SystemProfile::detect();
        let pre = grind_estimate(expected, profile.estimated_keys_per_sec(chain_parsed.id()), &pattern);
        if pre.risk == PatternRisk::Impractical && !force {
            return Err(
                "Pattern is not practical on a single machine. Shorten it or confirm to start anyway."
                    .to_string(),
            );
        }

        let cancel = CancelToken::new();
        *job = Some(cancel.clone());
        drop(job);

        std::thread::spawn(move || {
            run_grind_job(app, chain_parsed, pattern, profile, cancel);
        });
    }
    Ok(())
}

fn run_grind_job(app: AppHandle, chain: Chain, pattern: Pattern, mut profile: SystemProfile, cancel: CancelToken) {
    let _ = app.emit("grind-calibrating", ());

    if let Ok(rate) = benchmark(chain.clone(), &profile, BENCHMARK_SECS) {
        profile = profile.with_benchmark(rate);
        let expected = chain.expected_attempts(&pattern);
        let calibrated = grind_estimate(expected, profile.estimated_keys_per_sec(chain.id()), &pattern);
        let _ = app.emit(
            "grind-speed",
            SpeedPayload {
                rate,
                time_label: calibrated.time_label,
            },
        );
    }

    if cancel.is_cancelled() {
        let _ = app.emit("grind-cancelled", ());
        clear_job(&app);
        return;
    }

    let prefix_match = pattern.prefix_match.clone();
    let suffix_match = pattern.suffix_match.clone();
    let ignore_case = pattern.ignore_case;

    let result: Result<GrindResult, String> = grind(
        chain.clone(),
        pattern.clone(),
        &profile,
        &cancel,
        |attempts, rate, eta_min| {
            let _ = app.emit(
                "grind-progress",
                ProgressPayload {
                    attempts,
                    rate,
                    eta_min,
                },
            );
        },
    );

    match result {
        Ok(r) => {
            let exports = r
                .keypair
                .exports
                .into_iter()
                .map(|e| KeyExportPayload {
                    label: e.label,
                    value: e.value,
                    hint: e.hint,
                })
                .collect();
            let _ = app.emit(
                "grind-done",
                DonePayload {
                    address: r.keypair.address,
                    exports,
                    attempts: r.attempts,
                    elapsed_secs: r.elapsed_secs,
                    chain_display_name: chain.display_name().to_string(),
                    pattern_description: pattern.description(),
                    case_mode: pattern.case_mode().to_string(),
                    prefix_match,
                    suffix_match,
                    ignore_case,
                },
            );
        }
        Err(e) if e == "cancelled" => {
            let _ = app.emit("grind-cancelled", ());
        }
        Err(e) => {
            let _ = app.emit("grind-error", ErrorPayload { message: e });
        }
    }

    clear_job(&app);
}

fn clear_job(app: &AppHandle) {
    if let Some(state) = app.try_state::<AppState>() {
        if let Ok(mut job) = state.job.lock() {
            *job = None;
        }
    }
}

#[tauri::command]
pub fn stop_grind(state: State<AppState>) -> Result<(), String> {
    let job = state.job.lock().map_err(|_| "internal lock error".to_string())?;
    match job.as_ref() {
        Some(cancel) => {
            cancel.cancel();
            Ok(())
        }
        None => Err("No grind is running".to_string()),
    }
}

#[derive(serde::Deserialize)]
pub struct SaveExport {
    label: String,
    value: String,
}

#[derive(serde::Deserialize)]
pub struct SaveResultArgs {
    chain_display_name: String,
    pattern_description: String,
    case_mode: String,
    address: String,
    exports: Vec<SaveExport>,
    attempts: u64,
    elapsed_secs: f64,
}

#[tauri::command]
pub async fn save_result(app: AppHandle, payload: SaveResultArgs) -> Result<Option<String>, String> {
    let SaveResultArgs {
        chain_display_name,
        pattern_description,
        case_mode,
        address,
        exports,
        attempts,
        elapsed_secs,
    } = payload;

    let picked = app
        .dialog()
        .file()
        .set_file_name("vanity-results.txt")
        .add_filter("Text", &["txt"])
        .blocking_save_file();

    let Some(path) = picked else {
        return Ok(None);
    };
    let path = path
        .into_path()
        .map_err(|e| format!("invalid save path: {e}"))?;

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|e| format!("could not open file: {e}"))?;

    let timestamp = chrono_timestamp();
    writeln!(file, "━").ok();
    writeln!(file, "vanity-address match · {timestamp}").ok();
    writeln!(file, "chain:   {chain_display_name}").ok();
    writeln!(file, "target:  {pattern_description}").ok();
    writeln!(file, "mode:    {case_mode}").ok();
    writeln!(file, "stats:   {attempts} attempts in {elapsed_secs:.2}s").ok();
    writeln!(file).ok();
    writeln!(file, "Address: {address}").ok();
    for export in &exports {
        writeln!(file, "{}: {}", export.label, export.value).ok();
    }
    writeln!(file).ok();
    writeln!(
        file,
        "WARNING: This file contains private keys. Never commit or share it."
    )
    .ok();
    writeln!(file).ok();

    Ok(Some(path.display().to_string()))
}

fn chrono_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let z = (secs / 86_400) as i64 + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    let tod = secs % 86_400;
    let h = tod / 3600;
    let min = (tod % 3600) / 60;
    let s = tod % 60;
    format!("{y:04}-{m:02}-{d:02} {h:02}:{min:02}:{s:02} UTC")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_chains_covers_every_menu_entry() {
        let chains = list_chains();
        assert_eq!(chains.len(), MENU_CHAINS.len());
        assert_eq!(chains[0].id, "sol");
        assert!(chains.iter().any(|c| c.id == "evm"));
    }

    #[test]
    fn estimate_reports_pattern_and_risk() {
        let result = estimate("sol".to_string(), String::new(), "ax".to_string(), false).unwrap();
        assert!(result.pattern_description.contains("ax"));
        assert_eq!(result.risk, "None");
        assert!(result.length_guide.is_some());
    }

    #[test]
    fn estimate_flags_impractical_patterns() {
        let result = estimate(
            "sol".to_string(),
            String::new(),
            "akshaysingheavysuffix".to_string(),
            false,
        )
        .unwrap();
        assert_eq!(result.risk, "Impractical");
        assert!(result.warning.is_some());
    }

    #[test]
    fn estimate_rejects_empty_pattern() {
        let err = estimate("sol".to_string(), String::new(), String::new(), false).unwrap_err();
        assert!(err.contains("prefix"));
    }

    #[test]
    fn estimate_rejects_unsupported_exact_case() {
        let err = estimate("evm".to_string(), String::new(), "dead".to_string(), true).unwrap_err();
        assert!(err.contains("Exact case"));
    }

    #[test]
    fn system_profile_returns_workers() {
        let info = get_system_profile("sol".to_string()).unwrap();
        assert!(info.worker_description.contains("thread"));
        assert!(info.estimated_keys_per_sec > 0.0);
    }
}
