use super::base58_odds::Base58Layout;
use super::util::{
    grind_ed25519, keypair_from_secret, secret_from_attempt, Keypair, BASE58_ALPHABET,
};
use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::{matches_full, Pattern};

const BASE58_INVALID: &str = "0OIl";

/// A bare 32-byte public key, no version byte.
const LAYOUT: Base58Layout = Base58Layout {
    version: &[],
    random_len: 32,
    alphabet: BASE58_ALPHABET,
};

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

    fn format_json_byte_array(bytes: &[u8; 64]) -> String {
        let mut s = String::with_capacity(64 * 4 + 2);
        s.push('[');
        for (i, b) in bytes.iter().enumerate() {
            if i > 0 {
                s.push(',');
            }
            s.push_str(&b.to_string());
        }
        s.push(']');
        s
    }

    fn char_combinations(pattern: &str, ignore_case: bool) -> f64 {
        pattern
            .chars()
            .filter(|c| *c != '*')
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
            if c == '*' {
                continue;
            }
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
                    value: Self::format_json_byte_array(&keypair_bytes),
                    hint: Some("Phantom / Solflare / solana-keygen (byte array JSON)".into()),
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
        let prefix = prefix.unwrap_or("").to_string();
        let suffix = suffix.unwrap_or("").to_string();
        let contains = contains.unwrap_or("").to_string();

        if prefix.is_empty() && suffix.is_empty() && contains.is_empty() {
            return Err("Provide at least one of --prefix, --suffix, or --contains".into());
        }

        if !prefix.is_empty() {
            Self::validate_part("prefix", &prefix)?;
        }
        if !suffix.is_empty() {
            Self::validate_part("suffix", &suffix)?;
        }
        if !contains.is_empty() {
            Self::validate_part("contains", &contains)?;
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
        let contains_match = if ignore_case {
            contains.to_ascii_lowercase()
        } else {
            contains.clone()
        };

        let pattern = Pattern {
            prefix,
            suffix,
            contains,
            prefix_match,
            suffix_match,
            contains_match,
            ignore_case,
            fixed_prefix_len: 0,
        };
        LAYOUT.check_prefix("Solana", &pattern)?;
        Ok(pattern)
    }

    fn expected_attempts(&self, pattern: &Pattern) -> f64 {
        let mut combos = 1.0_f64;
        if pattern.has_prefix() {
            combos /= LAYOUT.prefix_probability(&pattern.prefix, pattern.ignore_case);
        }
        if pattern.has_suffix() {
            combos *= Self::char_combinations(&pattern.suffix, pattern.ignore_case);
        }
        if pattern.has_contains() {
            combos *= Self::char_combinations(&pattern.contains, pattern.ignore_case);
        }
        combos
    }

    fn matches(&self, address: &str, pattern: &Pattern) -> bool {
        matches_full(
            address,
            &pattern.prefix_match,
            &pattern.suffix_match,
            &pattern.contains_match,
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

/// SPL token mint address — same ed25519 keypair math as a wallet; the
/// exports are geared to `spl-token create-token <mint.json>`.
#[derive(Clone, Default)]
pub struct SolanaMintGrinder;

impl ChainGrinder for SolanaMintGrinder {
    fn id(&self) -> &'static str {
        "sol-mint"
    }

    fn display_name(&self) -> &'static str {
        "Solana token mint (SPL)"
    }

    fn grind_attempt(&self) -> (String, GrindAttempt) {
        SolanaGrinder.grind_attempt()
    }

    fn finalize(&self, attempt: GrindAttempt) -> KeypairResult {
        let secret_bytes = secret_from_attempt(attempt);
        let keypair = keypair_from_secret(secret_bytes);
        let keypair_bytes = SolanaGrinder::keypair_bytes(secret_bytes, &keypair);

        KeypairResult {
            address: SolanaGrinder::derive_address(&keypair),
            exports: vec![
                KeyExport {
                    label: "Mint Secret Key (base58)".into(),
                    value: bs58::encode(keypair_bytes).into_string(),
                    hint: Some("Keypair.fromSecretKey(bs58.decode(…)) in @solana/web3.js".into()),
                },
                KeyExport {
                    label: "Mint Keypair (JSON)".into(),
                    value: SolanaGrinder::format_json_byte_array(&keypair_bytes),
                    hint: Some(
                        "Save as mint.json → spl-token create-token mint.json (or --program-2022)"
                            .into(),
                    ),
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
        SolanaGrinder.build_pattern(prefix, suffix, contains, exact)
    }

    fn expected_attempts(&self, pattern: &Pattern) -> f64 {
        SolanaGrinder.expected_attempts(pattern)
    }

    fn matches(&self, address: &str, pattern: &Pattern) -> bool {
        SolanaGrinder.matches(address, pattern)
    }

    fn supports_exact_case(&self) -> bool {
        true
    }

    fn pattern_hint(&self) -> &'static str {
        "Base58 characters only. Invalid: 0, O, I, l (e.g. --suffix pump)"
    }
}

#[cfg(test)]
mod tests {
    use super::{SolanaGrinder, SolanaMintGrinder};
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
    fn mint_derives_same_address_as_wallet_for_seed() {
        let seed = [9u8; 32];
        let wallet = SolanaGrinder.finalize(crate::chain::GrindAttempt::Secret32(seed));
        let mint = SolanaMintGrinder.finalize(crate::chain::GrindAttempt::Secret32(seed));
        assert_eq!(mint.address, wallet.address);
        assert_eq!(mint.exports[0].value, wallet.exports[1].value);
        assert_eq!(mint.exports[1].value, wallet.exports[2].value);
    }

    #[test]
    fn solana_address_is_base58() {
        let g = SolanaGrinder;
        let (addr, _) = g.grind_attempt();
        assert!(bs58::decode(&addr).into_vec().is_ok());
    }
}
