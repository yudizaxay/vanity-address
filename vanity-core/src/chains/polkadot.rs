use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use blake2::{Blake2b512, Digest};
use solana_sdk::signature::{Keypair, Signer};

use super::util::{
    base58_combinations, build_base58_pattern, expected_from_pattern, grind_ed25519,
    keypair_from_secret, matches_pattern, secret_from_attempt, BASE58_ALPHABET,
};

/// Polkadot mainnet SS58 prefix.
const SS58_PREFIX: u8 = 0;

#[derive(Clone, Default)]
pub struct PolkadotGrinder;

impl PolkadotGrinder {
    fn encode_ss58(prefix: u8, pubkey: &[u8; 32]) -> String {
        let mut data = Vec::with_capacity(35);
        data.push(prefix);
        data.extend_from_slice(pubkey);

        let mut hasher = Blake2b512::new();
        hasher.update(b"SS58PRE");
        hasher.update(&data);
        let hash = hasher.finalize();
        data.extend_from_slice(&hash[..2]);

        bs58::encode(data).into_string()
    }

    fn derive_address(keypair: &Keypair) -> String {
        Self::encode_ss58(SS58_PREFIX, &keypair.pubkey().to_bytes())
    }
}

impl ChainGrinder for PolkadotGrinder {
    fn id(&self) -> &'static str {
        "dot"
    }

    fn display_name(&self) -> &'static str {
        "Polkadot (ed25519 SS58)"
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
                hint: Some("Polkadot.js / SubWallet (ed25519). sr25519 wallets may differ.".into()),
            }],
        }
    }

    fn build_pattern(
        &self,
        prefix: Option<&str>,
        suffix: Option<&str>,
        exact: bool,
    ) -> Result<Pattern, String> {
        build_base58_pattern(prefix, suffix, exact, BASE58_ALPHABET, 48)
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
        "SS58 base58 (Polkadot prefix 0, ed25519)."
    }
}

#[cfg(test)]
mod tests {
    use super::PolkadotGrinder;
    use crate::chain::ChainGrinder;

    #[test]
    fn polkadot_address_is_base58() {
        let g = PolkadotGrinder;
        let (addr, _) = g.grind_attempt();
        assert!(addr.len() >= 46);
        assert!(bs58::decode(&addr).into_vec().is_ok());
    }

    /// Known-answer test from substrate's own ed25519 SS58 vector (generic
    /// prefix 42): pubkey `0x1a0e2bf1e0195a1f5396c5fd209a620a48fe90f6f336d89c89405a0183a857a3`
    /// must encode to `5CesK3uTmn4NGfD3oyGBd1jrp4EfRyYdtqL3ERe9SXv8jUHb`. This
    /// verifies the SS58PRE/blake2b checksum against an independent
    /// implementation; the Polkadot mainnet prefix (0) is just a parameter
    /// on top of the same, now-verified encoder.
    #[test]
    fn ss58_encoding_matches_substrate_known_vector() {
        let pubkey: [u8; 32] =
            hex::decode("1a0e2bf1e0195a1f5396c5fd209a620a48fe90f6f336d89c89405a0183a857a3")
                .unwrap()
                .try_into()
                .unwrap();
        let addr = PolkadotGrinder::encode_ss58(42, &pubkey);
        assert_eq!(addr, "5CesK3uTmn4NGfD3oyGBd1jrp4EfRyYdtqL3ERe9SXv8jUHb");
    }
}
