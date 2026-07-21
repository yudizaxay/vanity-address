use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use bech32::{encode, Bech32, Hrp};
use solana_sdk::signature::{Keypair, Signer};

use super::util::{
    bech32_combinations, blake2b_var, build_base58_pattern, expected_from_pattern, grind_ed25519,
    keypair_from_secret, matches_pattern, secret_from_attempt, BECH32_CHARSET,
};

#[derive(Clone, Default)]
pub struct CardanoGrinder;

impl CardanoGrinder {
    /// Shelley enterprise address (payment key only), mainnet.
    /// Header = (type 6 << 4) | network 1 = 0x61.
    fn derive_address(keypair: &Keypair) -> String {
        let pubkey = keypair.pubkey().to_bytes();
        let payment_hash = blake2b_var(&pubkey, 28);
        let mut payload = Vec::with_capacity(29);
        payload.push(0x61);
        payload.extend_from_slice(&payment_hash);
        let hrp = Hrp::parse("addr").expect("valid hrp");
        encode::<Bech32>(hrp, &payload).expect("valid cardano address")
    }
}

impl ChainGrinder for CardanoGrinder {
    fn id(&self) -> &'static str {
        "ada"
    }

    fn display_name(&self) -> &'static str {
        "Cardano (enterprise)"
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
                hint: Some(
                    "Enterprise addr (payment only). Eternl/Nami may expect full CIP-1852 account."
                        .into(),
                ),
            }],
        }
    }

    fn build_pattern(
        &self,
        prefix: Option<&str>,
        suffix: Option<&str>,
        exact: bool,
    ) -> Result<Pattern, String> {
        let mut pattern = build_base58_pattern(prefix, suffix, exact, BECH32_CHARSET, 108)?;
        if pattern.has_prefix() && !pattern.prefix.starts_with("addr") {
            pattern.prefix = format!("addr1{}", pattern.prefix);
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
                .strip_prefix("addr1")
                .or_else(|| p.strip_prefix("addr"))
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
        "Bech32 after addr1 (Shelley enterprise / payment-only)."
    }
}

#[cfg(test)]
mod tests {
    use super::CardanoGrinder;
    use crate::chain::ChainGrinder;

    #[test]
    fn cardano_address_starts_with_addr1() {
        let g = CardanoGrinder;
        let (addr, _) = g.grind_attempt();
        assert!(addr.starts_with("addr1"));
    }
}
