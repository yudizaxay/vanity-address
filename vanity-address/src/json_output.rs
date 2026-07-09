//! Machine-readable JSON output for scripting (`--json`).

use serde::Serialize;
use std::io::{self, Write};
use vanity_core::{Chain, ChainGrinder, GrindResult, KeyExport, Pattern};

/// Stable error codes for automation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    InvalidChain,
    InvalidPattern,
    ImpracticalPattern,
    ExactNotSupported,
    InvalidThreads,
    GrindFailed,
    IoError,
    InvalidArgs,
}

#[derive(Debug, Serialize)]
pub struct ErrorPayload<'a> {
    pub error: &'a str,
    pub code: ErrorCode,
}

#[derive(Debug, Serialize)]
pub struct PatternPayload<'a> {
    pub prefix: &'a str,
    pub suffix: &'a str,
    pub description: &'a str,
    pub ignore_case: bool,
}

#[derive(Debug, Serialize)]
pub struct ExportPayload<'a> {
    pub label: &'a str,
    pub value: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct StatsPayload {
    pub attempts: u64,
    pub elapsed_secs: f64,
    pub keys_per_sec: f64,
}

#[derive(Debug, Serialize)]
pub struct SuccessPayload<'a> {
    pub version: &'static str,
    pub chain: &'a str,
    pub chain_name: &'a str,
    pub pattern: PatternPayload<'a>,
    pub address: &'a str,
    pub exports: Vec<ExportPayload<'a>>,
    pub stats: StatsPayload,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measured_keys_per_sec: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saved_to: Option<&'a str>,
}

pub fn print_error_json(msg: &str, code: ErrorCode) -> io::Result<()> {
    let payload = ErrorPayload { error: msg, code };
    let line = serde_json::to_string(&payload).map_err(|e| io::Error::other(e.to_string()))?;
    writeln!(io::stderr(), "{line}")
}

pub fn print_success_json(
    chain: &Chain,
    pattern: &Pattern,
    result: &GrindResult,
    measured_keys_per_sec: Option<f64>,
    saved_to: Option<&str>,
) -> io::Result<()> {
    let keys_per_sec = if result.elapsed_secs > 0.0 {
        result.attempts as f64 / result.elapsed_secs
    } else {
        0.0
    };

    let payload = SuccessPayload {
        version: env!("CARGO_PKG_VERSION"),
        chain: chain.id(),
        chain_name: chain.display_name(),
        pattern: PatternPayload {
            prefix: &pattern.prefix,
            suffix: &pattern.suffix,
            description: &pattern.description(),
            ignore_case: pattern.ignore_case,
        },
        address: &result.keypair.address,
        exports: result.keypair.exports.iter().map(export_payload).collect(),
        stats: StatsPayload {
            attempts: result.attempts,
            elapsed_secs: result.elapsed_secs,
            keys_per_sec,
        },
        measured_keys_per_sec,
        saved_to,
    };

    let line = serde_json::to_string(&payload).map_err(|e| io::Error::other(e.to_string()))?;
    writeln!(io::stdout(), "{line}")
}

fn export_payload(export: &KeyExport) -> ExportPayload<'_> {
    ExportPayload {
        label: &export.label,
        value: &export.value,
        hint: export.hint.as_deref(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vanity_core::{KeypairResult, SolanaGrinder};

    #[test]
    fn error_json_has_code_and_message() {
        let json = serde_json::to_string(&ErrorPayload {
            error: "unknown chain",
            code: ErrorCode::InvalidChain,
        })
        .unwrap();
        assert!(json.contains("\"code\":\"invalid_chain\""));
        assert!(json.contains("\"error\":\"unknown chain\""));
    }

    #[test]
    fn success_json_includes_exports_and_stats() {
        let chain = Chain::Solana(SolanaGrinder);
        let pattern = chain
            .build_pattern(None, Some("ax"), false)
            .expect("pattern");
        let result = GrindResult {
            keypair: KeypairResult {
                address: "7xKpQaxay".into(),
                exports: vec![KeyExport {
                    label: "Secret Key (base58)".into(),
                    value: "secret".into(),
                    hint: Some("hint".into()),
                }],
            },
            attempts: 1_000,
            elapsed_secs: 0.5,
        };

        let payload = SuccessPayload {
            version: "0.3.0",
            chain: chain.id(),
            chain_name: chain.display_name(),
            pattern: PatternPayload {
                prefix: &pattern.prefix,
                suffix: &pattern.suffix,
                description: &pattern.description(),
                ignore_case: pattern.ignore_case,
            },
            address: &result.keypair.address,
            exports: result.keypair.exports.iter().map(export_payload).collect(),
            stats: StatsPayload {
                attempts: result.attempts,
                elapsed_secs: result.elapsed_secs,
                keys_per_sec: 2000.0,
            },
            measured_keys_per_sec: Some(2_000_000.0),
            saved_to: Some("/tmp/out.txt"),
        };

        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("\"chain\":\"sol\""));
        assert!(json.contains("\"address\":\"7xKpQaxay\""));
        assert!(json.contains("\"saved_to\":\"/tmp/out.txt\""));
        assert!(json.contains("\"measured_keys_per_sec\":2000000"));
    }

    #[test]
    fn success_json_omits_optional_fields_when_none() {
        let chain = Chain::Solana(SolanaGrinder);
        let pattern = chain.build_pattern(None, Some("a"), false).unwrap();
        let result = GrindResult {
            keypair: KeypairResult {
                address: "addr".into(),
                exports: vec![],
            },
            attempts: 1,
            elapsed_secs: 1.0,
        };

        let payload = SuccessPayload {
            version: "0.3.0",
            chain: chain.id(),
            chain_name: chain.display_name(),
            pattern: PatternPayload {
                prefix: &pattern.prefix,
                suffix: &pattern.suffix,
                description: &pattern.description(),
                ignore_case: pattern.ignore_case,
            },
            address: &result.keypair.address,
            exports: vec![],
            stats: StatsPayload {
                attempts: 1,
                elapsed_secs: 1.0,
                keys_per_sec: 1.0,
            },
            measured_keys_per_sec: None,
            saved_to: None,
        };

        let json = serde_json::to_string(&payload).unwrap();
        assert!(!json.contains("saved_to"));
        assert!(!json.contains("measured_keys_per_sec"));
    }
}
