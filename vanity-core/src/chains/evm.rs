use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::{matches_both, Pattern};
use rand::rngs::OsRng;
use secp256k1::{Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};

#[derive(Clone, Default)]
pub struct EvmGrinder;

impl EvmGrinder {
    fn normalize_hex_pattern(input: &str) -> String {
        let stripped = input.strip_prefix("0x").unwrap_or(input);
        stripped.to_ascii_lowercase()
    }

    fn char_combinations(pattern: &str) -> f64 {
        let hex_len = pattern.strip_prefix("0x").unwrap_or(pattern).len();
        16f64.powi(hex_len as i32)
    }

    fn validate_part(label: &str, pattern: &str) -> Result<String, String> {
        let normalized = Self::normalize_hex_pattern(pattern);
        if normalized.is_empty() {
            return Err(format!("'{label}' cannot be empty"));
        }
        for c in normalized.chars() {
            if !c.is_ascii_hexdigit() {
                return Err(format!(
                    "'{label}' contains '{c}' — EVM patterns must be hex (0-9, a-f)"
                ));
            }
        }
        Ok(normalized)
    }

    fn derive_address(secret_key: &SecretKey) -> String {
        let secp = Secp256k1::new();
        let public_key = secret_key.public_key(&secp);
        let public_bytes = &public_key.serialize_uncompressed()[1..];
        let hash = Keccak256::digest(public_bytes);
        format!("0x{}", hex::encode(&hash[12..]))
    }
}

impl ChainGrinder for EvmGrinder {
    fn id(&self) -> &'static str {
        "evm"
    }

    fn display_name(&self) -> &'static str {
        "EVM (Ethereum)"
    }

    fn grind_attempt(&self) -> (String, GrindAttempt) {
        let mut rng = OsRng;
        let secret_key = SecretKey::new(&mut rng);
        let address = Self::derive_address(&secret_key);
        (address, GrindAttempt::Evm(secret_key.secret_bytes()))
    }

    fn finalize(&self, attempt: GrindAttempt) -> KeypairResult {
        let GrindAttempt::Evm(secret_bytes) = attempt else {
            panic!("evm finalize called with wrong attempt type");
        };
        let address = {
            let secret_key = SecretKey::from_slice(&secret_bytes)
                .expect("valid secp256k1 secret");
            Self::derive_address(&secret_key)
        };

        KeypairResult {
            address,
            exports: vec![
                KeyExport {
                    label: "Private Key (hex)".into(),
                    value: hex::encode(secret_bytes),
                    hint: Some("MetaMask / hardware wallet import".into()),
                },
                KeyExport {
                    label: "Private Key (0x hex)".into(),
                    value: format!("0x{}", hex::encode(secret_bytes)),
                    hint: Some("Standard EVM tooling format".into()),
                },
            ],
        }
    }

    fn build_pattern(
        &self,
        prefix: Option<&str>,
        suffix: Option<&str>,
        _exact: bool,
    ) -> Result<Pattern, String> {
        let prefix_raw = prefix.unwrap_or("").to_string();
        let suffix_raw = suffix.unwrap_or("").to_string();

        if prefix_raw.is_empty() && suffix_raw.is_empty() {
            return Err("Provide at least one of --prefix or --suffix".into());
        }

        let prefix = if prefix_raw.is_empty() {
            String::new()
        } else {
            let normalized = Self::validate_part("prefix", &prefix_raw)?;
            if normalized.len() > 40 {
                return Err("prefix is too long for a 40-character EVM address".into());
            }
            format!("0x{normalized}")
        };

        let suffix = if suffix_raw.is_empty() {
            String::new()
        } else {
            let normalized = Self::validate_part("suffix", &suffix_raw)?;
            if normalized.len() > 40 {
                return Err("suffix is too long for a 40-character EVM address".into());
            }
            normalized
        };

        let prefix_hex_len = prefix.strip_prefix("0x").unwrap_or(&prefix).len();
        if !prefix.is_empty() && !suffix.is_empty() && prefix_hex_len + suffix.len() > 40 {
            return Err(format!(
                "prefix + suffix length ({}) exceeds 40 hex characters",
                prefix_hex_len + suffix.len()
            ));
        }

        Ok(Pattern {
            prefix: prefix.clone(),
            suffix: suffix.clone(),
            prefix_match: prefix,
            suffix_match: suffix,
            ignore_case: true,
        })
    }

    fn expected_attempts(&self, pattern: &Pattern) -> f64 {
        let mut combos = 1.0_f64;
        if pattern.has_prefix() {
            combos *= Self::char_combinations(&pattern.prefix);
        }
        if pattern.has_suffix() {
            combos *= Self::char_combinations(&pattern.suffix);
        }
        combos
    }

    fn matches(&self, address: &str, pattern: &Pattern) -> bool {
        let addr = address.to_ascii_lowercase();
        matches_both(
            &addr,
            &pattern.prefix_match,
            &pattern.suffix_match,
            true,
        )
    }

    fn supports_exact_case(&self) -> bool {
        false
    }

    fn pattern_hint(&self) -> &'static str {
        "Hex characters (0-9, a-f). Optional 0x prefix."
    }
}

#[cfg(test)]
mod tests {
    use super::EvmGrinder;
    use crate::chain::ChainGrinder;

    #[test]
    fn evm_prefix_normalizes_0x() {
        let g = EvmGrinder;
        let p = g.build_pattern(Some("dead"), Some("beef"), false).unwrap();
        assert_eq!(p.prefix_match, "0xdead");
        assert_eq!(p.suffix_match, "beef");
    }

    #[test]
    fn evm_rejects_invalid_hex() {
        let g = EvmGrinder;
        assert!(g.build_pattern(Some("zzzz"), None, false).is_err());
    }

    #[test]
    fn evm_matches_address() {
        let g = EvmGrinder;
        let p = g.build_pattern(Some("dead"), Some("beef"), false).unwrap();
        assert!(g.matches(
            "0xdead000000000000000000000000000000beef",
            &p
        ));
        assert!(!g.matches(
            "0xbeef000000000000000000000000000000dead",
            &p
        ));
    }
}
