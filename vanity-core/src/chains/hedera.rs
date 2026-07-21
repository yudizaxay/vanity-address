use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use solana_sdk::signature::{Keypair, Signer};

use super::util::{
    build_hex_pattern, der_ed25519_spki, expected_from_pattern, grind_ed25519, hex_combinations,
    keypair_from_secret, matches_pattern, secret_from_attempt,
};

/// Hedera account IDs (`0.0.N`) are assigned on-chain. This grinder vanities the
/// ed25519 public key (hex) used when creating an account in HashPack / portal.
#[derive(Clone, Default)]
pub struct HederaGrinder;

impl HederaGrinder {
    fn derive_address(keypair: &Keypair) -> String {
        hex::encode(keypair.pubkey().to_bytes())
    }

    fn der_public_key(pubkey: &[u8; 32]) -> String {
        hex::encode(der_ed25519_spki(pubkey))
    }
}

impl ChainGrinder for HederaGrinder {
    fn id(&self) -> &'static str {
        "hedera"
    }

    fn display_name(&self) -> &'static str {
        "Hedera (ed25519 pubkey)"
    }

    fn grind_attempt(&self) -> (String, GrindAttempt) {
        grind_ed25519(Self::derive_address)
    }

    fn finalize(&self, attempt: GrindAttempt) -> KeypairResult {
        let secret_bytes = secret_from_attempt(attempt);
        let keypair = keypair_from_secret(secret_bytes);
        let pubkey = keypair.pubkey().to_bytes();
        let address = Self::derive_address(&keypair);

        KeypairResult {
            address,
            exports: vec![
                KeyExport {
                    label: "Private Key (hex)".into(),
                    value: hex::encode(secret_bytes),
                    hint: Some(
                        "Create account in HashPack / Hedera Portal — 0.0.N is assigned on-chain"
                            .into(),
                    ),
                },
                KeyExport {
                    label: "Public Key (DER hex)".into(),
                    value: Self::der_public_key(&pubkey),
                    hint: Some("Hedera DER SubjectPublicKeyInfo".into()),
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
        build_hex_pattern(prefix, suffix, false, 64)
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
        "Hex public key (account ID 0.0.N is assigned when you create the account)."
    }
}

#[cfg(test)]
mod tests {
    use super::HederaGrinder;
    use crate::chain::ChainGrinder;

    #[test]
    fn hedera_pubkey_hex_length() {
        let g = HederaGrinder;
        let (addr, _) = g.grind_attempt();
        assert_eq!(addr.len(), 64);
    }
}
