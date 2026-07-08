use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use rand::rngs::OsRng;
use secp256k1::{Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};

use super::util::{
    build_hex_pattern, expected_from_pattern, hex_combinations, matches_pattern,
    secret_from_attempt,
};

#[derive(Clone, Default)]
pub struct EvmGrinder;

impl EvmGrinder {
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
        (address, GrindAttempt::Secret32(secret_key.secret_bytes()))
    }

    fn finalize(&self, attempt: GrindAttempt) -> KeypairResult {
        let secret_bytes = secret_from_attempt(attempt);
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
        build_hex_pattern(prefix, suffix, true, 40)
    }

    fn expected_attempts(&self, pattern: &Pattern) -> f64 {
        expected_from_pattern(pattern, hex_combinations)
    }

    fn matches(&self, address: &str, pattern: &Pattern) -> bool {
        matches_pattern(address, pattern, true)
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
