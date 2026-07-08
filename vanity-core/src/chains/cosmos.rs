use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use bech32::{encode, Bech32, Hrp};
use secp256k1::SecretKey;

use super::util::{
    bech32_combinations, build_base58_pattern, expected_from_pattern, grind_secp256k1,
    hash160, matches_pattern, secret_from_attempt, BECH32_CHARSET,
};

#[derive(Clone)]
pub struct CosmosGrinder {
    pub id: &'static str,
    pub display_name: &'static str,
    pub hrp: &'static str,
}

impl CosmosGrinder {
    pub fn cosmos() -> Self {
        Self {
            id: "cosmos",
            display_name: "Cosmos (ATOM)",
            hrp: "cosmos",
        }
    }

    pub fn osmosis() -> Self {
        Self {
            id: "osmo",
            display_name: "Osmosis (OSMO)",
            hrp: "osmo",
        }
    }

    fn derive(&self, secret: &SecretKey) -> String {
        let secp = secp256k1::Secp256k1::new();
        let pubkey = secret.public_key(&secp).serialize();
        let hash = hash160(&pubkey);
        let hrp = Hrp::parse(self.hrp).expect("valid hrp");
        encode::<Bech32>(hrp, &hash).expect("valid bech32 address")
    }
}

impl ChainGrinder for CosmosGrinder {
    fn id(&self) -> &'static str {
        self.id
    }

    fn display_name(&self) -> &'static str {
        self.display_name
    }

    fn grind_attempt(&self) -> (String, GrindAttempt) {
        grind_secp256k1(|secret| self.derive(secret))
    }

    fn finalize(&self, attempt: GrindAttempt) -> KeypairResult {
        let secret_bytes = secret_from_attempt(attempt);
        let secret_key =
            SecretKey::from_slice(&secret_bytes).expect("valid secp256k1 secret");
        let address = self.derive(&secret_key);

        KeypairResult {
            address,
            exports: vec![
                KeyExport {
                    label: "Private Key (hex)".into(),
                    value: hex::encode(secret_bytes),
                    hint: Some("Keplr / Leap wallet import".into()),
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
        let mut pattern = build_base58_pattern(prefix, suffix, exact, BECH32_CHARSET, 38)?;
        // Match on the data portion after hrp + "1"
        let hrp_prefix = format!("{}1", self.hrp);
        if pattern.has_prefix() && !pattern.prefix.starts_with(&hrp_prefix) {
            pattern.prefix = format!("{hrp_prefix}{}", pattern.prefix);
            pattern.prefix_match = if pattern.ignore_case {
                pattern.prefix.to_ascii_lowercase()
            } else {
                pattern.prefix.clone()
            };
        }
        Ok(pattern)
    }

    fn expected_attempts(&self, pattern: &Pattern) -> f64 {
        expected_from_pattern(pattern, bech32_combinations)
    }

    fn matches(&self, address: &str, pattern: &Pattern) -> bool {
        matches_pattern(address, pattern, true)
    }

    fn supports_exact_case(&self) -> bool {
        false
    }

    fn pattern_hint(&self) -> &'static str {
        "Bech32 data chars, or full address including prefix (e.g. cosmos1…)."
    }
}
