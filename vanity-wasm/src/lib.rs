use serde::Serialize;
use vanity_core::{Chain, ChainGrinder};
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

fn js_err(code: &str, message: &str) -> JsValue {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"code".into(), &code.into()).ok();
    js_sys::Reflect::set(&obj, &"message".into(), &message.into()).ok();
    obj.into()
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
    let exact = !ignore_case;
    let pattern = chain
        .build_pattern(prefix_opt, suffix_opt, exact)
        .map_err(|e| js_err("INVALID_PATTERN", &e))?;

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
