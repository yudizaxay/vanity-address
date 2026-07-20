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
    fn derive_address(keypair: &Keypair) -> String {
        let pubkey = keypair.pubkey().to_bytes();
        let mut data = Vec::with_capacity(35);
        data.push(SS58_PREFIX);
        data.extend_from_slice(&pubkey);

        let mut hasher = Blake2b512::new();
        hasher.update(b"SS58PRE");
        hasher.update(&data);
        let hash = hasher.finalize();
        data.extend_from_slice(&hash[..2]);

        bs58::encode(data).into_string()
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
}
