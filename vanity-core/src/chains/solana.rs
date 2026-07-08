use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::{matches_both, Pattern};
use solana_sdk::signature::{Keypair, Signer};

const BASE58_INVALID: &str = "0OIl";

#[derive(Clone, Default)]
pub struct SolanaGrinder;

impl SolanaGrinder {
    fn char_combinations(pattern: &str, ignore_case: bool) -> f64 {
        pattern
            .chars()
            .map(|c| {
                if ignore_case && c.is_ascii_alphabetic() {
                    29.0
                } else {
                    58.0
                }
            })
            .product()
    }

    fn validate_part(label: &str, pattern: &str) -> Result<(), String> {
        for c in pattern.chars() {
            if BASE58_INVALID.contains(c) || !c.is_ascii_alphanumeric() {
                return Err(format!(
                    "'{label}' contains '{c}', which never appears in a Solana base58 address"
                ));
            }
        }
        Ok(())
    }
}

impl ChainGrinder for SolanaGrinder {
    fn id(&self) -> &'static str {
        "sol"
    }

    fn display_name(&self) -> &'static str {
        "Solana"
    }

    fn grind_attempt(&self) -> (String, GrindAttempt) {
        let keypair = Keypair::new();
        let address = keypair.pubkey().to_string();
        (address, GrindAttempt::Solana(keypair))
    }

    fn finalize(&self, attempt: GrindAttempt) -> KeypairResult {
        let GrindAttempt::Solana(keypair) = attempt else {
            panic!("solana finalize called with wrong attempt type");
        };
        let address = keypair.pubkey().to_string();

        KeypairResult {
            address,
            exports: vec![
                KeyExport {
                    label: "Private Key (hex)".into(),
                    value: hex::encode(keypair.secret().to_bytes()),
                    hint: Some("Raw 32-byte secret".into()),
                },
                KeyExport {
                    label: "Private Key (base58)".into(),
                    value: bs58::encode(keypair.to_bytes()).into_string(),
                    hint: Some("Phantom / Solflare wallet import".into()),
                },
                KeyExport {
                    label: "Keypair (JSON)".into(),
                    value: format!("{:?}", keypair.to_bytes().to_vec()),
                    hint: Some("solana-cli format".into()),
                },
            ],
        }
    }

    fn build_pattern(
        &self,
        prefix: Option<&str>,
        suffix: Option<&str>,
        exact: bool,
    ) -> Result<Pattern, String> {
        let prefix = prefix.unwrap_or("").to_string();
        let suffix = suffix.unwrap_or("").to_string();

        if prefix.is_empty() && suffix.is_empty() {
            return Err("Provide at least one of --prefix or --suffix".into());
        }

        if !prefix.is_empty() {
            Self::validate_part("prefix", &prefix)?;
        }
        if !suffix.is_empty() {
            Self::validate_part("suffix", &suffix)?;
        }

        let ignore_case = !exact;
        let prefix_match = if ignore_case {
            prefix.to_ascii_lowercase()
        } else {
            prefix.clone()
        };
        let suffix_match = if ignore_case {
            suffix.to_ascii_lowercase()
        } else {
            suffix.clone()
        };

        Ok(Pattern {
            prefix,
            suffix,
            prefix_match,
            suffix_match,
            ignore_case,
        })
    }

    fn expected_attempts(&self, pattern: &Pattern) -> f64 {
        let mut combos = 1.0_f64;
        if pattern.has_prefix() {
            combos *= Self::char_combinations(&pattern.prefix, pattern.ignore_case);
        }
        if pattern.has_suffix() {
            combos *= Self::char_combinations(&pattern.suffix, pattern.ignore_case);
        }
        combos
    }

    fn matches(&self, address: &str, pattern: &Pattern) -> bool {
        matches_both(
            address,
            &pattern.prefix_match,
            &pattern.suffix_match,
            pattern.ignore_case,
        )
    }

    fn supports_exact_case(&self) -> bool {
        true
    }

    fn pattern_hint(&self) -> &'static str {
        "Base58 characters only. Invalid: 0, O, I, l"
    }
}
