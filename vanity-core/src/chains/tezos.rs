use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use solana_sdk::signature::{Keypair, Signer};

use super::util::{
    base58_check_encode_raw, base58_combinations, blake2b_var, build_base58_pattern,
    expected_from_pattern, grind_ed25519, keypair_from_secret, matches_pattern,
    secret_from_attempt, BASE58_ALPHABET,
};

/// tz1 = ed25519 public key hash (Blake2b-160) with Tezos prefix.
const TZ1_PREFIX: [u8; 3] = [6, 161, 159];
/// edsk = ed25519 seed prefix.
const EDSK_PREFIX: [u8; 4] = [43, 246, 78, 7];

#[derive(Clone, Default)]
pub struct TezosGrinder;

impl TezosGrinder {
    fn derive_address(keypair: &Keypair) -> String {
        let pubkey = keypair.pubkey().to_bytes();
        let pkh = blake2b_var(&pubkey, 20);
        let mut payload = Vec::with_capacity(23);
        payload.extend_from_slice(&TZ1_PREFIX);
        payload.extend_from_slice(&pkh);
        base58_check_encode_raw(&payload)
    }

    fn encode_edsk(secret: [u8; 32]) -> String {
        let mut payload = Vec::with_capacity(36);
        payload.extend_from_slice(&EDSK_PREFIX);
        payload.extend_from_slice(&secret);
        base58_check_encode_raw(&payload)
    }
}

impl ChainGrinder for TezosGrinder {
    fn id(&self) -> &'static str {
        "xtz"
    }

    fn display_name(&self) -> &'static str {
        "Tezos (tz1)"
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
            exports: vec![
                KeyExport {
                    label: "Secret Key (edsk…)".into(),
                    value: Self::encode_edsk(secret_bytes),
                    hint: Some("Temple / Kukai wallet import".into()),
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
        let mut pattern = build_base58_pattern(prefix, suffix, exact, BASE58_ALPHABET, 36)?;
        if pattern.has_prefix() && !pattern.prefix.starts_with("tz1") {
            pattern.prefix = format!("tz1{}", pattern.prefix);
            pattern.prefix_match = if pattern.ignore_case {
                pattern.prefix.to_ascii_lowercase()
            } else {
                pattern.prefix.clone()
            };
        }
        Ok(pattern)
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
        "Base58 — tz1 (ed25519) addresses."
    }
}

#[cfg(test)]
mod tests {
    use super::TezosGrinder;
    use crate::chain::ChainGrinder;

    #[test]
    fn tezos_address_starts_with_tz1() {
        let g = TezosGrinder;
        let (addr, _) = g.grind_attempt();
        assert!(addr.starts_with("tz1"));
    }
}
