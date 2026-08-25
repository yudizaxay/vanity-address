use super::util::{grind_ed25519, keypair_from_secret, secret_from_attempt, Keypair};
use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::{matches_both, Pattern};

const BASE58_INVALID: &str = "0OIl";

#[derive(Clone, Default)]
pub struct SolanaGrinder;

impl SolanaGrinder {
    fn derive_address(keypair: &Keypair) -> String {
        bs58::encode(keypair.pubkey().to_bytes()).into_string()
    }

    /// solana_sdk's `Keypair::to_bytes()` / `solana-keygen` JSON format:
    /// 32-byte secret followed by the 32-byte public key.
    fn keypair_bytes(secret: [u8; 32], keypair: &Keypair) -> [u8; 64] {
        let mut bytes = [0u8; 64];
        bytes[..32].copy_from_slice(&secret);
        bytes[32..].copy_from_slice(&keypair.pubkey().to_bytes());
        bytes
    }

    fn char_combinations(pattern: &str, ignore_case: bool) -> f64 {
        pattern
            .chars()
            .map(|c| {
                if ignore_case && c.is_ascii_alphabetic() {
                    29.0
                } else {
                    58.0
                }
            })
            .product()
    }

    fn validate_part(label: &str, pattern: &str) -> Result<(), String> {
        for c in pattern.chars() {
            if BASE58_INVALID.contains(c) || !c.is_ascii_alphanumeric() {
                return Err(format!(
                    "'{label}' contains '{c}', which never appears in a Solana base58 address"
                ));
            }
        }
        Ok(())
    }
}

impl ChainGrinder for SolanaGrinder {
    fn id(&self) -> &'static str {
        "sol"
    }

    fn display_name(&self) -> &'static str {
        "Solana"
    }

    fn grind_attempt(&self) -> (String, GrindAttempt) {
        grind_ed25519(Self::derive_address)
    }

    fn finalize(&self, attempt: GrindAttempt) -> KeypairResult {
        let secret_bytes = secret_from_attempt(attempt);
        let keypair = keypair_from_secret(secret_bytes);
        let address = Self::derive_address(&keypair);
        let keypair_bytes = Self::keypair_bytes(secret_bytes, &keypair);

        KeypairResult {
            address,
            exports: vec![
                KeyExport {
                    label: "Private Key (hex)".into(),
                    value: hex::encode(secret_bytes),
                    hint: Some("Raw 32-byte secret".into()),
                },
                KeyExport {
                    label: "Private Key (base58)".into(),
                    value: bs58::encode(keypair_bytes).into_string(),
                    hint: Some("Phantom / Solflare wallet import".into()),
                },
                KeyExport {
                    label: "Keypair (JSON)".into(),
                    value: format!("{:?}", keypair_bytes.to_vec()),
                    hint: Some("solana-cli format".into()),
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
        let prefix = prefix.unwrap_or("").to_string();
        let suffix = suffix.unwrap_or("").to_string();

        if prefix.is_empty() && suffix.is_empty() {
            return Err("Provide at least one of --prefix or --suffix".into());
        }

        if !prefix.is_empty() {
            Self::validate_part("prefix", &prefix)?;
        }
        if !suffix.is_empty() {
            Self::validate_part("suffix", &suffix)?;
        }

        let ignore_case = !exact;
        let prefix_match = if ignore_case {
            prefix.to_ascii_lowercase()
        } else {
            prefix.clone()
        };
        let suffix_match = if ignore_case {
            suffix.to_ascii_lowercase()
        } else {
            suffix.clone()
        };

        Ok(Pattern {
            prefix,
            suffix,
            prefix_match,
            suffix_match,
            ignore_case,
        })
    }

    fn expected_attempts(&self, pattern: &Pattern) -> f64 {
        let mut combos = 1.0_f64;
        if pattern.has_prefix() {
            combos *= Self::char_combinations(&pattern.prefix, pattern.ignore_case);
        }
        if pattern.has_suffix() {
            combos *= Self::char_combinations(&pattern.suffix, pattern.ignore_case);
        }
        combos
    }

    fn matches(&self, address: &str, pattern: &Pattern) -> bool {
        matches_both(
            address,
            &pattern.prefix_match,
            &pattern.suffix_match,
            pattern.ignore_case,
        )
    }

    fn supports_exact_case(&self) -> bool {
        true
    }

    fn pattern_hint(&self) -> &'static str {
        "Base58 characters only. Invalid: 0, O, I, l"
    }
}

#[cfg(test)]
mod tests {
    use super::SolanaGrinder;
    use crate::chain::ChainGrinder;
    use crate::chains::util::keypair_from_secret;
    use solana_sdk::signature::SeedDerivable;

    /// Byte-for-byte cross-check against the old `solana_sdk`-backed
    /// implementation this replaced: same 32-byte seed must produce the
    /// same base58 address and the same 64-byte keypair export. Proves the
    /// migration to the local ed25519-dalek wrapper is behavior-preserving.
    #[test]
    fn matches_solana_sdk_for_fixed_seed() {
        let seed = [7u8; 32];

        let old_keypair = solana_sdk::signature::Keypair::from_seed(&seed).unwrap();
        let old_address = solana_sdk::signer::Signer::pubkey(&old_keypair).to_string();
        let old_keypair_bytes = old_keypair.to_bytes();

        let new_keypair = keypair_from_secret(seed);
        let new_address = SolanaGrinder::derive_address(&new_keypair);
        let new_keypair_bytes = SolanaGrinder::keypair_bytes(seed, &new_keypair);

        assert_eq!(new_address, old_address, "address mismatch");
        assert_eq!(
            new_keypair_bytes.to_vec(),
            old_keypair_bytes.to_vec(),
            "keypair bytes mismatch"
        );
    }

    #[test]
    fn solana_address_is_base58() {
        let g = SolanaGrinder;
        let (addr, _) = g.grind_attempt();
        assert!(bs58::decode(&addr).into_vec().is_ok());
    }
}
