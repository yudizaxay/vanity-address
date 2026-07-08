use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use solana_sdk::signature::{Keypair, Signer};
use stellar_strkey::ed25519::{PrivateKey, PublicKey};

use super::util::{
    base58_combinations, build_base58_pattern, expected_from_pattern, grind_ed25519,
    keypair_from_secret, matches_pattern, secret_from_attempt, BASE58_ALPHABET,
};

#[derive(Clone, Default)]
pub struct StellarGrinder;

impl StellarGrinder {
    fn derive_address(keypair: &Keypair) -> String {
        let pubkey = keypair.pubkey().to_bytes();
        PublicKey(pubkey).to_string()
    }
}

impl ChainGrinder for StellarGrinder {
    fn id(&self) -> &'static str {
        "xlm"
    }

    fn display_name(&self) -> &'static str {
        "Stellar (XLM)"
    }

    fn grind_attempt(&self) -> (String, GrindAttempt) {
        grind_ed25519(Self::derive_address)
    }

    fn finalize(&self, attempt: GrindAttempt) -> KeypairResult {
        let secret_bytes = secret_from_attempt(attempt);
        let keypair = keypair_from_secret(secret_bytes);
        let address = Self::derive_address(&keypair);
        let seed = PrivateKey(secret_bytes);

        KeypairResult {
            address,
            exports: vec![
                KeyExport {
                    label: "Secret Key (S…)".into(),
                    value: seed.to_string(),
                    hint: Some("Stellar / Lobstr wallet import".into()),
                },
                KeyExport {
                    label: "Private Key (hex)".into(),
                    value: hex::encode(secret_bytes),
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
        build_base58_pattern(prefix, suffix, exact, BASE58_ALPHABET, 56)
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
        "Base58 strkey — public addresses start with G."
    }
}

#[cfg(test)]
mod tests {
    use super::StellarGrinder;
    use crate::chain::ChainGrinder;

    #[test]
    fn stellar_address_starts_with_g() {
        let g = StellarGrinder;
        let (addr, _) = g.grind_attempt();
        assert!(addr.starts_with('G'));
    }
}
