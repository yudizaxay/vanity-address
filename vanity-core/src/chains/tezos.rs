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
/// edsk = ed25519 32-byte seed prefix. (Not to be confused with the
/// same-looking "edsk" prefix `[43, 246, 78, 7]` used for the 64-byte
/// *expanded* secret key — a different, longer format. Using that prefix
/// on a 32-byte seed produces a string wallets won't recognize as a valid
/// key. Verified against pytezos's own test vector below.)
const EDSK_PREFIX: [u8; 4] = [13, 15, 58, 7];

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
    use super::{TezosGrinder, EDSK_PREFIX};
    use crate::chain::ChainGrinder;
    use solana_sdk::signature::{Keypair, SeedDerivable};

    #[test]
    fn tezos_address_starts_with_tz1() {
        let g = TezosGrinder;
        let (addr, _) = g.grind_attempt();
        assert!(addr.starts_with("tz1"));
    }

    /// Known-answer test from pytezos: the seed encoded by
    /// `edsk3nM41ygNfSxVU4w1uAW3G9EnTQEB5rjojeZedLTGmiGRcierVv` must derive
    /// public key `edpku976gpuAD2bXyx1XGraeKuCo1gUZ3LAJcHM12W1ecxZwoiu22R` and
    /// address `tz1eKkWU5hGtfLUiqNpucHrXymm83z3DG9Sq`. This also confirms
    /// `EDSK_PREFIX` round-trips through an independent implementation.
    #[test]
    fn tezos_matches_pytezos_known_vector() {
        let seed: [u8; 32] =
            hex::decode("92542d866a5263115aa416fd3e1dce4ced35f5545417d1f73763f7093552a72b")
                .unwrap()[..32]
                .try_into()
                .unwrap();
        let keypair = Keypair::from_seed(&seed).unwrap();
        let address = TezosGrinder::derive_address(&keypair);
        assert_eq!(address, "tz1eKkWU5hGtfLUiqNpucHrXymm83z3DG9Sq");

        let edsk = TezosGrinder::encode_edsk(seed);
        assert_eq!(
            edsk,
            "edsk3nM41ygNfSxVU4w1uAW3G9EnTQEB5rjojeZedLTGmiGRcierVv"
        );
        assert_eq!(EDSK_PREFIX, [13, 15, 58, 7]);
    }
}
