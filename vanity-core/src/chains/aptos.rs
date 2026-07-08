use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use sha3::{Digest, Sha3_256};
use solana_sdk::signature::{Keypair, Signer};

use super::util::{
    build_hex_pattern, expected_from_pattern, grind_ed25519, hex_combinations, matches_pattern,
    secret_from_attempt,
};

#[derive(Clone, Default)]
pub struct AptosGrinder;

impl AptosGrinder {
    fn derive_address(keypair: &Keypair) -> String {
        let pubkey = keypair.pubkey().to_bytes();
        let mut hasher = Sha3_256::new();
        hasher.update(pubkey);
        hasher.update([0u8]);
        let hash = hasher.finalize();
        format!("0x{}", hex::encode(hash))
    }
}

impl ChainGrinder for AptosGrinder {
    fn id(&self) -> &'static str {
        "aptos"
    }

    fn display_name(&self) -> &'static str {
        "Aptos"
    }

    fn grind_attempt(&self) -> (String, GrindAttempt) {
        grind_ed25519(Self::derive_address)
    }

    fn finalize(&self, attempt: GrindAttempt) -> KeypairResult {
        let secret_bytes = secret_from_attempt(attempt);
        let keypair = super::util::keypair_from_secret(secret_bytes);
        let address = Self::derive_address(&keypair);

        KeypairResult {
            address,
            exports: vec![KeyExport {
                label: "Private Key (hex)".into(),
                value: hex::encode(secret_bytes),
                hint: Some("Petra / Martian wallet import".into()),
            }],
        }
    }

    fn build_pattern(
        &self,
        prefix: Option<&str>,
        suffix: Option<&str>,
        _exact: bool,
    ) -> Result<Pattern, String> {
        build_hex_pattern(prefix, suffix, true, 64)
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
