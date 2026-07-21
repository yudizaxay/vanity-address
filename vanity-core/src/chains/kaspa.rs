use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use secp256k1::{Keypair as SecpKeypair, Secp256k1, SecretKey};

use super::util::{
    bech32_combinations, build_base58_pattern, expected_from_pattern, grind_secp256k1,
    kaspa_address_data, matches_pattern, secret_from_attempt, BECH32_CHARSET,
};

/// PubKey (Schnorr, 32-byte x-only) address version, per kaspa-addresses.
const VERSION_PUBKEY: u8 = 0;

#[derive(Clone, Default)]
pub struct KaspaGrinder;

impl KaspaGrinder {
    fn derive(secret: &SecretKey) -> String {
        let secp = Secp256k1::new();
        let keypair = SecpKeypair::from_secret_key(&secp, secret);
        let (xonly, _) = keypair.x_only_public_key();
        let data = kaspa_address_data("kaspa", VERSION_PUBKEY, &xonly.serialize());
        format!("kaspa:{data}")
    }
}

impl ChainGrinder for KaspaGrinder {
    fn id(&self) -> &'static str {
        "kaspa"
    }

    fn display_name(&self) -> &'static str {
        "Kaspa"
    }

    fn grind_attempt(&self) -> (String, GrindAttempt) {
        grind_secp256k1(Self::derive)
    }

    fn finalize(&self, attempt: GrindAttempt) -> KeypairResult {
        let secret_bytes = secret_from_attempt(attempt);
        let secret_key = SecretKey::from_slice(&secret_bytes).expect("valid secp256k1 secret");
        let address = Self::derive(&secret_key);

        KeypairResult {
            address,
            exports: vec![KeyExport {
                label: "Private Key (hex)".into(),
                value: hex::encode(secret_bytes),
                hint: Some("Kaspium / kaspa wallet import".into()),
            }],
        }
    }

    fn build_pattern(
        &self,
        prefix: Option<&str>,
        suffix: Option<&str>,
        exact: bool,
    ) -> Result<Pattern, String> {
        let mut pattern = build_base58_pattern(prefix, suffix, exact, BECH32_CHARSET, 72)?;
        if pattern.has_prefix() && !pattern.prefix.starts_with("kaspa:") {
            pattern.prefix = format!("kaspa:{}", pattern.prefix);
            pattern.prefix_match = if pattern.ignore_case {
                pattern.prefix.to_ascii_lowercase()
            } else {
                pattern.prefix.clone()
            };
        }
        Ok(pattern)
    }

    fn expected_attempts(&self, pattern: &Pattern) -> f64 {
        expected_from_pattern(pattern, |p| {
            let data = p.strip_prefix("kaspa:").unwrap_or(p);
            bech32_combinations(data)
        })
    }

    fn matches(&self, address: &str, pattern: &Pattern) -> bool {
        matches_pattern(address, pattern, true)
    }

    fn supports_exact_case(&self) -> bool {
        false
    }

    fn pattern_hint(&self) -> &'static str {
        "Bech32 after kaspa: (q, p, z, r…)."
    }
}

#[cfg(test)]
mod tests {
    use super::super::util::kaspa_address_data;
    use super::{KaspaGrinder, VERSION_PUBKEY};
    use crate::chain::ChainGrinder;

    /// From kaspa-addresses' own test vectors (rusty-kaspa
    /// crypto/addresses/src/lib.rs): an all-zero 32-byte PubKey payload on
    /// mainnet must encode to this exact address. Confirms the checksum
    /// algorithm (a CashAddr-style BCH code, not standard bech32) is correct.
    #[test]
    fn kaspa_checksum_matches_known_vector() {
        let addr = format!(
            "kaspa:{}",
            kaspa_address_data("kaspa", VERSION_PUBKEY, &[0u8; 32])
        );
        assert_eq!(
            addr,
            "kaspa:qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqkx9awp4e"
        );
    }

    #[test]
    fn kaspa_address_prefix() {
        let g = KaspaGrinder;
        let (addr, _) = g.grind_attempt();
        assert!(addr.starts_with("kaspa:"));
        assert_eq!(addr.len(), "kaspa:".len() + 61);
    }
}
