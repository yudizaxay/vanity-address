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
    fn derive(secret: &SecretKey) -> String {
        let secp = Secp256k1::new();
        let pubkey: PublicKey = secret.public_key(&secp);
        let uncompressed = pubkey.serialize_uncompressed();
        let payload = blake2b_var(&uncompressed, 20);
        let mut checksum_input = Vec::with_capacity(21);
        checksum_input.push(1u8); // protocol
        checksum_input.extend_from_slice(&payload);
        let checksum = blake2b_var(&checksum_input, 4);
        let mut encoded = Vec::with_capacity(24);
        encoded.extend_from_slice(&payload);
        encoded.extend_from_slice(&checksum);
        format!("f1{}", BASE32_NOPAD.encode(&encoded).to_ascii_lowercase())
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
}
