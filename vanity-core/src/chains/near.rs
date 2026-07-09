use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use solana_sdk::signature::{Keypair, Signer};

use super::util::{
    build_hex_pattern, expected_from_pattern, grind_ed25519, hex_combinations, keypair_from_secret,
    matches_pattern, secret_from_attempt,
};

#[derive(Clone, Default)]
pub struct NearGrinder;

impl NearGrinder {
    fn derive_address(keypair: &Keypair) -> String {
        hex::encode(keypair.pubkey().to_bytes())
    }
}

impl ChainGrinder for NearGrinder {
    fn id(&self) -> &'static str {
        "near"
    }

    fn display_name(&self) -> &'static str {
        "NEAR"
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
                hint: Some("NEAR Wallet / implicit account".into()),
            }],
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
        "Hex (implicit NEAR account = ed25519 public key)."
    }
}
