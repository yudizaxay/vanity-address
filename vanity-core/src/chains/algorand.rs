use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use data_encoding::BASE32_NOPAD;
use sha2::{Digest, Sha512_256};
use solana_sdk::signature::{Keypair, Signer};

use super::util::{
    base32_combinations, build_base58_pattern, expected_from_pattern, grind_ed25519,
    keypair_from_secret, matches_pattern, secret_from_attempt, BASE32_ALPHABET,
};

#[derive(Clone, Default)]
pub struct AlgorandGrinder;

impl AlgorandGrinder {
    fn derive_address(keypair: &Keypair) -> String {
        let pubkey = keypair.pubkey().to_bytes();
        let hash = Sha512_256::digest(pubkey);
        let mut payload = [0u8; 36];
        payload[..32].copy_from_slice(&pubkey);
        payload[32..].copy_from_slice(&hash[28..]);
        BASE32_NOPAD.encode(&payload)
    }
}

impl ChainGrinder for AlgorandGrinder {
    fn id(&self) -> &'static str {
        "algo"
    }

    fn display_name(&self) -> &'static str {
        "Algorand"
    }

    fn grind_attempt(&self) -> (String, GrindAttempt) {
        grind_ed25519(Self::derive_address)
    }

    fn finalize(&self, attempt: GrindAttempt) -> KeypairResult {
        let secret_bytes = secret_from_attempt(attempt);
        let keypair = keypair_from_secret(secret_bytes);
        let address = Self::derive_address(&keypair);

        KeypairResult {
            address,
            exports: vec![KeyExport {
                label: "Private Key (hex)".into(),
                value: hex::encode(secret_bytes),
                hint: Some("Pera / Defly wallet import".into()),
            }],
        }
    }

    fn build_pattern(
        &self,
        prefix: Option<&str>,
        suffix: Option<&str>,
        exact: bool,
    ) -> Result<Pattern, String> {
        let upper = |s: Option<&str>| s.map(|v| v.to_ascii_uppercase());
        build_base58_pattern(
            upper(prefix).as_deref(),
            upper(suffix).as_deref(),
            exact,
            BASE32_ALPHABET,
            58,
        )
    }

    fn expected_attempts(&self, pattern: &Pattern) -> f64 {
        expected_from_pattern(pattern, base32_combinations)
    }

    fn matches(&self, address: &str, pattern: &Pattern) -> bool {
        matches_pattern(address, pattern, false)
    }

    fn supports_exact_case(&self) -> bool {
        true
    }

    fn pattern_hint(&self) -> &'static str {
        "Base32 (A-Z, 2-7). Algorand addresses are 58 characters."
    }
}

#[cfg(test)]
mod tests {
    use super::AlgorandGrinder;
    use crate::chain::ChainGrinder;
    use solana_sdk::signature::{Keypair, SeedDerivable};

    #[test]
    fn algo_address_length() {
        let g = AlgorandGrinder;
        let (addr, _) = g.grind_attempt();
        assert_eq!(addr.len(), 58);
        assert!(addr
            .chars()
            .all(|c| c.is_ascii_uppercase() || ('2'..='7').contains(&c)));
    }

    /// Known-answer test from bip_utils (verified against the official
    /// Algorand wallet): the all-zero 32-byte seed must derive this exact
    /// address.
    #[test]
    fn algo_matches_zero_seed_known_vector() {
        let seed = [0u8; 32];
        let keypair = Keypair::from_seed(&seed).unwrap();
        let addr = AlgorandGrinder::derive_address(&keypair);
        assert_eq!(
            addr,
            "HNVCPPGOW2SC2YVDVDICU3YNONSTEFLXDXREHJR2YBEKDC2Z3IUZSC6YGI"
        );
    }
}
