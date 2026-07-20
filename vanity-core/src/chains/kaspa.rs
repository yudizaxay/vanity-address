use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use bech32::{encode, Bech32, Hrp};
use secp256k1::{Keypair as SecpKeypair, Secp256k1, SecretKey};

use super::util::{
    bech32_combinations, build_base58_pattern, expected_from_pattern, grind_secp256k1,
    matches_pattern, secret_from_attempt, BECH32_CHARSET,
};

#[derive(Clone, Default)]
pub struct KaspaGrinder;

impl KaspaGrinder {
    fn derive(secret: &SecretKey) -> String {
        let secp = Secp256k1::new();
        let keypair = SecpKeypair::from_secret_key(&secp, secret);
        let (xonly, _) = keypair.x_only_public_key();
        let mut payload = Vec::with_capacity(33);
        payload.push(0u8); // PubKey version
        payload.extend_from_slice(&xonly.serialize());
        let hrp = Hrp::parse("kaspa").expect("valid hrp");
        // Kaspa uses `kaspa:` (colon) instead of BIP-173 `kaspa1` separator.
        let bip173 = encode::<Bech32>(hrp, &payload).expect("valid kaspa payload");
        let data = bip173.strip_prefix("kaspa1").expect("bech32 hrp separator");
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
    use super::KaspaGrinder;
    use crate::chain::ChainGrinder;

    #[test]
    fn kaspa_address_prefix() {
        let g = KaspaGrinder;
        let (addr, _) = g.grind_attempt();
        assert!(addr.starts_with("kaspa:"));
    }
}
