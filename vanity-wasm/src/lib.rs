use serde::Serialize;
use vanity_core::{
    default_keys_per_sec, grind_estimate, Chain, ChainGrinder, PatternRisk, MENU_CHAINS,
};
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct ExportOut {
    label: String,
    value: String,
    hint: Option<String>,
}

#[derive(Serialize)]
struct FoundOut {
    address: String,
    exports: Vec<ExportOut>,
}

#[derive(Serialize)]
struct ChunkOut {
    found: bool,
    attempts: u32,
    result: Option<FoundOut>,
}

#[derive(Serialize)]
struct ChainInfoOut {
    id: String,
    name: String,
}

#[derive(Serialize)]
struct EstimateOut {
    attempts: f64,
    attempts_label: String,
    avg_secs: f64,
    time_label: String,
    difficulty: String,
    difficulty_bars: String,
    risk: String,
    pattern_chars: usize,
    keys_per_sec: f64,
}

#[derive(Serialize)]
struct ValidateOut {
    ok: bool,
    error: Option<String>,
    risk: Option<String>,
    attempts: Option<f64>,
    attempts_label: Option<String>,
    time_label: Option<String>,
}

fn js_err(code: &str, message: &str) -> JsValue {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"code".into(), &code.into()).ok();
    js_sys::Reflect::set(&obj, &"message".into(), &message.into()).ok();
    obj.into()
}

fn build_pattern(
    chain: &Chain,
    prefix: &str,
    suffix: &str,
    ignore_case: bool,
) -> Result<vanity_core::Pattern, JsValue> {
    let prefix_opt = if prefix.is_empty() {
        None
    } else {
        Some(prefix)
    };
    let suffix_opt = if suffix.is_empty() {
        None
    } else {
        Some(suffix)
    };
    if prefix_opt.is_none() && suffix_opt.is_none() {
        return Err(js_err("INVALID_PATTERN", "prefix or suffix is required"));
    }
    let exact = !ignore_case;
    chain
        .build_pattern(prefix_opt, suffix_opt, exact)
        .map_err(|e| js_err("INVALID_PATTERN", &e))
}

/// Runs up to `attempts` grind tries for `chain_id` against the given
/// prefix/suffix pattern, returning as soon as a match is found or the
/// attempt budget is exhausted. Called repeatedly from JS so control
/// returns between chunks (progress reporting, cancellation).
#[wasm_bindgen]
pub fn grind_chunk(
    chain_id: &str,
    prefix: &str,
    suffix: &str,
    ignore_case: bool,
    attempts: u32,
) -> Result<JsValue, JsValue> {
    let chain = Chain::from_id(chain_id).map_err(|e| js_err("INVALID_CHAIN", &e))?;
    let pattern = build_pattern(&chain, prefix, suffix, ignore_case)?;

    for i in 0..attempts {
        let (address, attempt) = chain.grind_attempt();
        if chain.matches(&address, &pattern) {
            let kp = chain.finalize(attempt);
            let out = ChunkOut {
                found: true,
                attempts: i + 1,
                result: Some(FoundOut {
                    address: kp.address,
                    exports: kp
                        .exports
                        .into_iter()
                        .map(|e| ExportOut {
                            label: e.label,
                            value: e.value,
                            hint: e.hint,
                        })
                        .collect(),
                }),
            };
            return serde_wasm_bindgen::to_value(&out)
                .map_err(|e| js_err("INTERNAL", &e.to_string()));
        }
    }

    let out = ChunkOut {
        found: false,
        attempts,
        result: None,
    };
    serde_wasm_bindgen::to_value(&out).map_err(|e| js_err("INTERNAL", &e.to_string()))
}

/// True if `chain_id` (or a known alias like `eth` → EVM) is supported.
#[wasm_bindgen]
pub fn is_valid_chain(chain_id: &str) -> bool {
    Chain::from_id(chain_id).is_ok()
}

/// All supported chain ids + display names (A–Z menu order).
#[wasm_bindgen]
pub fn list_chains() -> Result<JsValue, JsValue> {
    let list: Vec<ChainInfoOut> = MENU_CHAINS
        .iter()
        .map(|(id, name)| ChainInfoOut {
            id: (*id).to_string(),
            name: (*name).to_string(),
        })
        .collect();
    serde_wasm_bindgen::to_value(&list).map_err(|e| js_err("INTERNAL", &e.to_string()))
}

/// Estimate expected attempts / ETA for a pattern.
/// Pass `keys_per_sec <= 0` to use the single-thread heuristic for that chain.
#[wasm_bindgen]
pub fn estimate_difficulty(
    chain_id: &str,
    prefix: &str,
    suffix: &str,
    ignore_case: bool,
    keys_per_sec: f64,
) -> Result<JsValue, JsValue> {
    let chain = Chain::from_id(chain_id).map_err(|e| js_err("INVALID_CHAIN", &e))?;
    let pattern = build_pattern(&chain, prefix, suffix, ignore_case)?;
    let attempts = chain.expected_attempts(&pattern);
    let kps = if keys_per_sec > 0.0 {
        keys_per_sec
    } else {
        default_keys_per_sec(chain.id())
    };
    let est = grind_estimate(attempts, kps, &pattern);
    let out = EstimateOut {
        attempts: est.attempts,
        attempts_label: est.attempts_label,
        avg_secs: est.avg_secs,
        time_label: est.time_label,
        difficulty: est.difficulty.to_string(),
        difficulty_bars: est.difficulty_bars,
        risk: est.risk.as_str().to_string(),
        pattern_chars: est.pattern_chars,
        keys_per_sec: kps,
    };
    serde_wasm_bindgen::to_value(&out).map_err(|e| js_err("INTERNAL", &e.to_string()))
}

/// Validate chain + pattern; on success also returns a quick risk hint.
#[wasm_bindgen]
pub fn validate_pattern(
    chain_id: &str,
    prefix: &str,
    suffix: &str,
    ignore_case: bool,
) -> Result<JsValue, JsValue> {
    let chain = match Chain::from_id(chain_id) {
        Ok(c) => c,
        Err(e) => {
            let out = ValidateOut {
                ok: false,
                error: Some(e),
                risk: None,
                attempts: None,
                attempts_label: None,
                time_label: None,
            };
            return serde_wasm_bindgen::to_value(&out)
                .map_err(|err| js_err("INTERNAL", &err.to_string()));
        }
    };

    let pattern = match build_pattern(&chain, prefix, suffix, ignore_case) {
        Ok(p) => p,
        Err(js) => {
            let message = js_sys::Reflect::get(&js, &"message".into())
                .ok()
                .and_then(|v| v.as_string())
                .unwrap_or_else(|| "invalid pattern".to_string());
            let out = ValidateOut {
                ok: false,
                error: Some(message),
                risk: None,
                attempts: None,
                attempts_label: None,
                time_label: None,
            };
            return serde_wasm_bindgen::to_value(&out)
                .map_err(|err| js_err("INTERNAL", &err.to_string()));
        }
    };

    let attempts = chain.expected_attempts(&pattern);
    let kps = default_keys_per_sec(chain.id());
    let est = grind_estimate(attempts, kps, &pattern);
    let risk = if est.risk == PatternRisk::None {
        None
    } else {
        Some(est.risk.as_str().to_string())
    };
    let out = ValidateOut {
        ok: true,
        error: None,
        risk,
        attempts: Some(est.attempts),
        attempts_label: Some(est.attempts_label),
        time_label: Some(est.time_label),
    };
    serde_wasm_bindgen::to_value(&out).map_err(|e| js_err("INTERNAL", &e.to_string()))
}

#[cfg(test)]
mod tests {
    use vanity_core::{Chain, ChainGrinder};

    #[test]
    fn evm_grind_attempt_matches_own_pattern() {
        let chain = Chain::from_id("evm").expect("evm chain");
        let pattern = chain
            .build_pattern(Some("a"), None, false)
            .expect("build pattern");
        let mut found = false;
        for _ in 0..20_000 {
            let (address, _attempt) = chain.grind_attempt();
            if chain.matches(&address, &pattern) {
                found = true;
                break;
            }
        }
        assert!(
            found,
            "expected a match within 20k attempts for a 1-char prefix"
        );
    }
}
