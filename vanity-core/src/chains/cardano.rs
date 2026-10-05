use super::util::Keypair;
use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use bech32::{encode, Bech32, Hrp};

use super::util::{
    address_start_factor, apply_address_start, bech32_combinations, blake2b_var,
    build_base58_pattern, expected_from_pattern, grind_ed25519, keypair_from_secret,
    matches_pattern, secret_from_attempt, BECH32_CHARSET,
};

/// Header 0x61 packs as `v`, then 3 header bits + 2 hash bits → one of `y 9 x 8`.
const CARDANO_NEXT: &[&str] = &["v", "89xy"];

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
        contains: Option<&str>,
        exact: bool,
    ) -> Result<Pattern, String> {
        let mut pattern =
            build_base58_pattern(prefix, suffix, contains, exact, BECH32_CHARSET, 108)?;
        apply_address_start(&mut pattern, "Cardano", "addr1", CARDANO_NEXT)?;
        Ok(pattern)
    }

    fn expected_attempts(&self, pattern: &Pattern) -> f64 {
        expected_from_pattern(pattern, bech32_combinations)
            * address_start_factor(pattern, CARDANO_NEXT, 32.0)
    }

    fn matches(&self, address: &str, pattern: &Pattern) -> bool {
        matches_pattern(address, pattern, true)
    }

    fn supports_exact_case(&self) -> bool {
        false
    }

    fn pattern_hint(&self) -> &'static str {
        "Bech32. Addresses are addr1v then 8/9/x/y — start your prefix with e.g. v8…"
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
