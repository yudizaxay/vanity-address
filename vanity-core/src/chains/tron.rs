use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use secp256k1::SecretKey;
use sha3::{Digest, Keccak256};

use super::util::{
    base58_check_encode_raw, base58_combinations, build_base58_pattern, expected_from_pattern,
    grind_secp256k1, matches_pattern, secret_from_attempt, BASE58_ALPHABET,
};

#[derive(Clone, Default)]
pub struct TronGrinder;

impl TronGrinder {
    fn derive_address(secret: &SecretKey) -> String {
        let secp = secp256k1::Secp256k1::new();
        let public_key = secret.public_key(&secp);
        let public_bytes = &public_key.serialize_uncompressed()[1..];
        let hash = Keccak256::digest(public_bytes);
        let mut payload = vec![0x41u8];
        payload.extend_from_slice(&hash[12..]);
        base58_check_encode_raw(&payload)
    }
}

impl ChainGrinder for TronGrinder {
    fn id(&self) -> &'static str {
        "trx"
    }

    fn display_name(&self) -> &'static str {
        "Tron (TRX)"
    }

    fn grind_attempt(&self) -> (String, GrindAttempt) {
        grind_secp256k1(Self::derive_address)
    }

    fn finalize(&self, attempt: GrindAttempt) -> KeypairResult {
        let secret_bytes = secret_from_attempt(attempt);
        let secret_key =
            SecretKey::from_slice(&secret_bytes).expect("valid secp256k1 secret");
        let address = Self::derive_address(&secret_key);

        KeypairResult {
            address,
            exports: vec![
                KeyExport {
                    label: "Private Key (hex)".into(),
                    value: hex::encode(secret_bytes),
                    hint: Some("TronLink / TronGrid import".into()),
                },
                KeyExport {
                    label: "Private Key (0x hex)".into(),
                    value: format!("0x{}", hex::encode(secret_bytes)),
                    hint: None,
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
        build_base58_pattern(prefix, suffix, exact, BASE58_ALPHABET, 34)
    }

    fn expected_attempts(&self, pattern: &Pattern) -> f64 {
        expected_from_pattern(pattern, base58_combinations)
    }

    fn matches(&self, address: &str, pattern: &Pattern) -> bool {
        matches_pattern(address, pattern, false)
    }

    fn supports_exact_case(&self) -> bool {
        true
    }

    fn pattern_hint(&self) -> &'static str {
        "Base58 — Tron addresses start with T."
    }
}
