use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use secp256k1::SecretKey;
use sha2::{Digest, Sha256};

use super::util::{
    base58_combinations, build_base58_pattern, encode_base58_with_alphabet, expected_from_pattern,
    grind_secp256k1, hash160, matches_pattern, secret_from_attempt, RIPPLE_ALPHABET,
};

#[derive(Clone, Default)]
pub struct RippleGrinder;

impl RippleGrinder {
    fn derive_address(secret: &SecretKey) -> String {
        let secp = secp256k1::Secp256k1::new();
        let pubkey = secret.public_key(&secp).serialize();
        let account_id = hash160(&pubkey);
        let mut payload = vec![0x00];
        payload.extend_from_slice(&account_id);
        let checksum = &Sha256::digest(Sha256::digest(&payload))[..4];
        payload.extend_from_slice(checksum);
        encode_base58_with_alphabet(&payload, RIPPLE_ALPHABET)
    }
}

impl ChainGrinder for RippleGrinder {
    fn id(&self) -> &'static str {
        "xrp"
    }

    fn display_name(&self) -> &'static str {
        "Ripple (XRP)"
    }

    fn grind_attempt(&self) -> (String, GrindAttempt) {
        grind_secp256k1(Self::derive_address)
    }

    fn finalize(&self, attempt: GrindAttempt) -> KeypairResult {
        let secret_bytes = secret_from_attempt(attempt);
        let secret_key = SecretKey::from_slice(&secret_bytes).expect("valid secp256k1 secret");
        let address = Self::derive_address(&secret_key);

        KeypairResult {
            address,
            exports: vec![KeyExport {
                label: "Private Key (hex)".into(),
                value: hex::encode(secret_bytes),
                hint: Some("XUMM / compatible XRP wallet".into()),
            }],
        }
    }

    fn build_pattern(
        &self,
        prefix: Option<&str>,
        suffix: Option<&str>,
        exact: bool,
    ) -> Result<Pattern, String> {
        build_base58_pattern(prefix, suffix, exact, RIPPLE_ALPHABET, 35)
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
        "Ripple base58 alphabet — addresses start with r."
    }
}

#[cfg(test)]
mod tests {
    use super::RippleGrinder;
    use crate::chain::ChainGrinder;
    use crate::chains::util::RIPPLE_ALPHABET;

    #[test]
    fn ripple_alphabet_is_full_base58() {
        assert_eq!(RIPPLE_ALPHABET.len(), 58);
        assert_eq!(
            RIPPLE_ALPHABET
                .chars()
                .collect::<std::collections::HashSet<_>>()
                .len(),
            58
        );
    }

    #[test]
    fn ripple_addresses_start_with_r_and_never_panic() {
        let g = RippleGrinder;
        for _ in 0..200 {
            let (addr, attempt) = g.grind_attempt();
            assert!(addr.starts_with('r'), "{addr}");
            let result = g.finalize(attempt);
            assert_eq!(result.address, addr);
        }
    }
}
