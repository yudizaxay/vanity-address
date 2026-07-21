use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use data_encoding::BASE32_NOPAD;
use secp256k1::{PublicKey, Secp256k1, SecretKey};

use super::util::{
    base32_combinations, blake2b_var, build_base58_pattern, expected_from_pattern, grind_secp256k1,
    matches_pattern, secret_from_attempt, BASE32_ALPHABET_LOWER,
};

#[derive(Clone, Default)]
pub struct FilecoinGrinder;

impl FilecoinGrinder {
    /// Protocol 1 (secp256k1) address: f1 + base32(payload || checksum).
    fn address_from_uncompressed_pubkey(uncompressed: &[u8]) -> String {
        let payload = blake2b_var(uncompressed, 20);
        let mut checksum_input = Vec::with_capacity(21);
        checksum_input.push(1u8); // protocol
        checksum_input.extend_from_slice(&payload);
        let checksum = blake2b_var(&checksum_input, 4);
        let mut encoded = Vec::with_capacity(24);
        encoded.extend_from_slice(&payload);
        encoded.extend_from_slice(&checksum);
        format!("f1{}", BASE32_NOPAD.encode(&encoded).to_ascii_lowercase())
    }

    fn derive(secret: &SecretKey) -> String {
        let secp = Secp256k1::new();
        let pubkey: PublicKey = secret.public_key(&secp);
        Self::address_from_uncompressed_pubkey(&pubkey.serialize_uncompressed())
    }
}

impl ChainGrinder for FilecoinGrinder {
    fn id(&self) -> &'static str {
        "fil"
    }

    fn display_name(&self) -> &'static str {
        "Filecoin (f1)"
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
                hint: Some("Lotus / Glif secp256k1 (f1) import".into()),
            }],
        }
    }

    fn build_pattern(
        &self,
        prefix: Option<&str>,
        suffix: Option<&str>,
        exact: bool,
    ) -> Result<Pattern, String> {
        let lower = |s: Option<&str>| s.map(|v| v.to_ascii_lowercase());
        let mut pattern = build_base58_pattern(
            lower(prefix).as_deref(),
            lower(suffix).as_deref(),
            exact,
            BASE32_ALPHABET_LOWER,
            64,
        )?;
        if pattern.has_prefix() && !pattern.prefix.starts_with("f1") {
            let rest = pattern.prefix.strip_prefix('f').unwrap_or(&pattern.prefix);
            pattern.prefix = format!("f1{rest}");
            pattern.prefix_match = pattern.prefix.clone();
        }
        Ok(pattern)
    }

    fn expected_attempts(&self, pattern: &Pattern) -> f64 {
        expected_from_pattern(pattern, |p| {
            let data = p.strip_prefix("f1").unwrap_or(p);
            base32_combinations(data)
        })
    }

    fn matches(&self, address: &str, pattern: &Pattern) -> bool {
        matches_pattern(address, pattern, true)
    }

    fn supports_exact_case(&self) -> bool {
        false
    }

    fn pattern_hint(&self) -> &'static str {
        "Base32 after f1 (secp256k1 protocol-1 addresses)."
    }
}

#[cfg(test)]
mod tests {
    use super::FilecoinGrinder;
    use crate::chain::ChainGrinder;

    #[test]
    fn filecoin_address_starts_with_f1() {
        let g = FilecoinGrinder;
        let (addr, _) = g.grind_attempt();
        assert!(addr.starts_with("f1"));
    }

    /// Known-answer test from filecoin-project/go-address's own
    /// TestVectorSecp256k1Address: this exact uncompressed pubkey must
    /// produce this exact mainnet f1 address.
    #[test]
    fn filecoin_matches_go_address_known_vector() {
        let uncompressed: [u8; 65] = [
            4, 148, 2, 250, 195, 126, 100, 50, 164, 22, 163, 160, 202, 84, 38, 181, 24, 90, 179,
            178, 79, 97, 52, 239, 162, 92, 228, 135, 200, 45, 46, 78, 19, 191, 69, 37, 17, 224,
            210, 36, 84, 33, 248, 97, 59, 193, 13, 114, 250, 33, 102, 102, 169, 108, 59, 193, 57,
            32, 211, 255, 35, 63, 208, 188, 5,
        ];
        let addr = FilecoinGrinder::address_from_uncompressed_pubkey(&uncompressed);
        assert_eq!(addr, "f15ihq5ibzwki2b4ep2f46avlkrqzhpqgtga7pdrq");
    }
}
