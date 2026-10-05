use super::util::Keypair;
use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use stellar_strkey::ed25519::{PrivateKey, PublicKey};

use super::util::{
    address_start_factor, apply_address_start, base32_combinations, build_base58_pattern,
    expected_from_pattern, grind_ed25519, keypair_from_secret, matches_pattern,
    secret_from_attempt, BASE32_ALPHABET,
};

/// Version byte 6<<3 leaves only 2 key bits in the second base32 char → `A`–`D`.
const STELLAR_NEXT: &[&str] = &["ABCD"];

#[derive(Clone, Default)]
pub struct StellarGrinder;

impl StellarGrinder {
    fn derive_address(keypair: &Keypair) -> String {
        let pubkey = keypair.pubkey().to_bytes();
        PublicKey(pubkey).to_string()
    }
}

impl ChainGrinder for StellarGrinder {
    fn id(&self) -> &'static str {
        "xlm"
    }

    fn display_name(&self) -> &'static str {
        "Stellar (XLM)"
    }

    fn grind_attempt(&self) -> (String, GrindAttempt) {
        grind_ed25519(Self::derive_address)
    }

    fn finalize(&self, attempt: GrindAttempt) -> KeypairResult {
        let secret_bytes = secret_from_attempt(attempt);
        let keypair = keypair_from_secret(secret_bytes);
        let address = Self::derive_address(&keypair);
        let seed = PrivateKey(secret_bytes);

        KeypairResult {
            address,
            exports: vec![
                KeyExport {
                    label: "Secret Key (S…)".into(),
                    value: seed.to_string(),
                    hint: Some("Stellar / Lobstr wallet import".into()),
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
        contains: Option<&str>,
        exact: bool,
    ) -> Result<Pattern, String> {
        let upper = |s: Option<&str>| s.map(|v| v.to_ascii_uppercase());
        let mut pattern = build_base58_pattern(
            upper(prefix).as_deref(),
            upper(suffix).as_deref(),
            upper(contains).as_deref(),
            exact,
            BASE32_ALPHABET,
            56,
        )?;
        apply_address_start(&mut pattern, "Stellar", "G", STELLAR_NEXT)?;
        Ok(pattern)
    }

    fn expected_attempts(&self, pattern: &Pattern) -> f64 {
        expected_from_pattern(pattern, base32_combinations)
            * address_start_factor(pattern, STELLAR_NEXT, 32.0)
    }

    fn matches(&self, address: &str, pattern: &Pattern) -> bool {
        matches_pattern(address, pattern, false)
    }

    fn supports_exact_case(&self) -> bool {
        true
    }

    fn pattern_hint(&self) -> &'static str {
        "Base32 (A-Z, 2-7). Addresses are G then A/B/C/D — start your prefix with e.g. A…"
    }
}

#[cfg(test)]
mod tests {
    use super::StellarGrinder;
    use crate::chain::ChainGrinder;

    #[test]
    fn stellar_address_starts_with_g() {
        let g = StellarGrinder;
        let (addr, _) = g.grind_attempt();
        assert!(addr.starts_with('G'));
    }
}
