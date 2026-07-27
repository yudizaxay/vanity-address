use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use bech32::{encode, Bech32, Hrp};
use solana_sdk::signature::{Keypair, Signer};

use super::util::{
    bech32_combinations, build_base58_pattern, expected_from_pattern, grind_ed25519,
    keypair_from_secret, matches_pattern, secret_from_attempt, BECH32_CHARSET,
};

#[derive(Clone, Default)]
pub struct MultiversXGrinder;

impl MultiversXGrinder {
    /// MultiversX account address = bech32(HRP `erd`, 32-byte ed25519 pubkey).
    fn encode_erd(pubkey: &[u8; 32]) -> String {
        let hrp = Hrp::parse("erd").expect("valid hrp");
        encode::<Bech32>(hrp, pubkey).expect("valid bech32 address")
    }

    fn derive_address(keypair: &Keypair) -> String {
        Self::encode_erd(&keypair.pubkey().to_bytes())
    }
}

impl ChainGrinder for MultiversXGrinder {
    fn id(&self) -> &'static str {
        "erd"
    }

    fn display_name(&self) -> &'static str {
        "MultiversX (erd1)"
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
                hint: Some("xPortal / MultiversX Wallet (ed25519 seed hex)".into()),
            }],
        }
    }

    fn build_pattern(
        &self,
        prefix: Option<&str>,
        suffix: Option<&str>,
        exact: bool,
    ) -> Result<Pattern, String> {
        let mut pattern = build_base58_pattern(prefix, suffix, exact, BECH32_CHARSET, 62)?;
        if pattern.has_prefix() && !pattern.prefix.starts_with("erd") {
            pattern.prefix = format!("erd1{}", pattern.prefix);
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
            let data = p
                .strip_prefix("erd1")
                .or_else(|| p.strip_prefix("erd"))
                .unwrap_or(p);
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
        "Bech32 (erd1… = ed25519 pubkey). Prefer --suffix; addresses always start with erd1."
    }
}

#[cfg(test)]
mod tests {
    use super::MultiversXGrinder;
    use crate::chain::ChainGrinder;

    #[test]
    fn multiversx_address_starts_with_erd1() {
        let g = MultiversXGrinder;
        let (addr, _) = g.grind_attempt();
        assert!(addr.starts_with("erd1"));
        assert_eq!(addr.len(), 62);
    }

    /// Alice test wallet pubkey from MultiversX SDK examples.
    #[test]
    fn erd_encoding_matches_alice_known_vector() {
        let pubkey: [u8; 32] =
            hex::decode("0139472eff6886771a982f3083da5d421f24c29181e63888228dc81ca60d69e1")
                .unwrap()
                .try_into()
                .unwrap();
        let addr = MultiversXGrinder::encode_erd(&pubkey);
        assert_eq!(
            addr,
            "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th"
        );
    }
}
