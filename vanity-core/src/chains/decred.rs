//! Decred mainnet P2PKH (`Ds…`).
//!
//! Hash160 = RIPEMD160(BLAKE256(compressed pubkey));
//! Base58Check with 2-byte version `[0x07, 0x3f]` and double-BLAKE256 checksum.

use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use blake_hash::{Blake256, Digest as _};
use ripemd::Ripemd160;
use secp256k1::{Secp256k1, SecretKey};
use sha2::Digest;

use super::base58_odds::Base58Layout;
use super::util::{
    build_base58_pattern, grind_secp256k1, matches_pattern, secret_from_attempt, BASE58_ALPHABET,
};

const DS_PREFIX: [u8; 2] = [0x07, 0x3f];
/// Decred mainnet compressed WIF network id (`Pm…`).
const WIF_PREFIX: [u8; 2] = [0x22, 0xde];

const LAYOUT: Base58Layout = Base58Layout {
    version: &DS_PREFIX,
    random_len: 24,
    alphabet: BASE58_ALPHABET,
};

#[derive(Clone, Default)]
pub struct DecredGrinder;

impl DecredGrinder {
    fn hash160_blake(pubkey: &[u8]) -> [u8; 20] {
        let blake = Blake256::digest(pubkey);
        Ripemd160::digest(blake).into()
    }

    fn derive(secret: &SecretKey) -> String {
        let secp = Secp256k1::new();
        let hash = Self::hash160_blake(&secret.public_key(&secp).serialize());
        let mut payload = Vec::with_capacity(26);
        payload.extend_from_slice(&DS_PREFIX);
        payload.extend_from_slice(&hash);
        let c1 = Blake256::digest(&payload);
        let c2 = Blake256::digest(&c1);
        payload.extend_from_slice(&c2[..4]);
        bs58::encode(payload).into_string()
    }

    fn wif_encode(secret: [u8; 32]) -> String {
        let mut payload = Vec::with_capacity(39);
        payload.extend_from_slice(&WIF_PREFIX);
        payload.extend_from_slice(&secret);
        payload.push(0x01);
        let c1 = Blake256::digest(&payload);
        let c2 = Blake256::digest(&c1);
        payload.extend_from_slice(&c2[..4]);
        bs58::encode(payload).into_string()
    }
}

impl ChainGrinder for DecredGrinder {
    fn id(&self) -> &'static str {
        "dcr"
    }

    fn display_name(&self) -> &'static str {
        "Decred (Ds… P2PKH)"
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
            exports: vec![
                KeyExport {
                    label: "Private Key (hex)".into(),
                    value: hex::encode(secret_bytes),
                    hint: Some("Decrediton / dcrwallet".into()),
                },
                KeyExport {
                    label: "Private Key (WIF)".into(),
                    value: Self::wif_encode(secret_bytes),
                    hint: Some("Decred compressed WIF (Pm…)".into()),
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
        let mut pattern =
            build_base58_pattern(prefix, suffix, contains, exact, BASE58_ALPHABET, 35)?;
        LAYOUT.apply_lead(self.display_name(), &mut pattern, "Ds")?;
        Ok(pattern)
    }

    fn expected_attempts(&self, pattern: &Pattern) -> f64 {
        LAYOUT.expected_attempts(pattern)
    }

    fn matches(&self, address: &str, pattern: &Pattern) -> bool {
        matches_pattern(address, pattern, false)
    }

    fn supports_exact_case(&self) -> bool {
        true
    }

    fn pattern_hint(&self) -> &'static str {
        "Base58. Mainnet P2PKH always starts with Ds (added for you)."
    }
}

#[cfg(test)]
mod tests {
    use super::DecredGrinder;
    use crate::chain::ChainGrinder;

    #[test]
    fn addresses_start_with_ds() {
        let g = DecredGrinder;
        for _ in 0..16 {
            let (addr, attempt) = g.grind_attempt();
            assert!(addr.starts_with("Ds"), "got {addr}");
            assert_eq!(g.finalize(attempt).address, addr);
        }
    }

    #[test]
    fn wif_starts_with_pm() {
        let g = DecredGrinder;
        let (_addr, attempt) = g.grind_attempt();
        let result = g.finalize(attempt);
        let wif = &result.exports[1].value;
        assert!(wif.starts_with("Pm"), "got {wif}");
    }
}
