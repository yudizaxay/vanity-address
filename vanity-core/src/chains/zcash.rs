//! Zcash transparent P2PKH (`t1…`) — secp256k1 + hash160 + Base58Check.
//!
//! Mainnet prefix is **two bytes** `[0x1C, 0xB8]` (not a single version byte).
//! Shielded Sapling (`zs1…`) / Unified addresses are out of scope.

use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use secp256k1::{Secp256k1, SecretKey};
use sha2::{Digest, Sha256};

use super::base58_odds::Base58Layout;
use super::util::{
    build_base58_pattern, grind_secp256k1, hash160, matches_pattern, secret_from_attempt,
    BASE58_ALPHABET,
};

/// Mainnet transparent P2PKH version (Zcash Protocol Spec §5.6.1.1).
const T1_PREFIX: [u8; 2] = [0x1c, 0xb8];
/// Same WIF version as Bitcoin mainnet for transparent keys.
const WIF_BYTE: u8 = 0x80;

const LAYOUT: Base58Layout = Base58Layout {
    version: &T1_PREFIX,
    random_len: 24,
    alphabet: BASE58_ALPHABET,
};

#[derive(Clone, Default)]
pub struct ZcashGrinder;

impl ZcashGrinder {
    fn derive(secret: &SecretKey) -> String {
        let secp = Secp256k1::new();
        let hash = hash160(&secret.public_key(&secp).serialize());
        let mut payload = Vec::with_capacity(26);
        payload.extend_from_slice(&T1_PREFIX);
        payload.extend_from_slice(&hash);
        let checksum = &Sha256::digest(Sha256::digest(&payload))[..4];
        payload.extend_from_slice(checksum);
        bs58::encode(payload).into_string()
    }

    fn wif_encode(secret: [u8; 32]) -> String {
        let mut payload = vec![WIF_BYTE];
        payload.extend_from_slice(&secret);
        payload.push(0x01);
        super::util::base58_check_encode_raw(&payload)
    }
}

impl ChainGrinder for ZcashGrinder {
    fn id(&self) -> &'static str {
        "zec"
    }

    fn display_name(&self) -> &'static str {
        "Zcash (transparent t1)"
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
                    hint: Some("Transparent key — import into Zcash / YWallet / similar".into()),
                },
                KeyExport {
                    label: "Private Key (WIF)".into(),
                    value: Self::wif_encode(secret_bytes),
                    hint: Some("Compressed WIF (Bitcoin-compatible version byte)".into()),
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
        LAYOUT.apply_lead(self.display_name(), &mut pattern, "t1")?;
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
        "Base58 transparent t1… only (no zs1 shielded / Unified). Lead t1 is added for you."
    }
}

#[cfg(test)]
mod tests {
    use super::ZcashGrinder;
    use crate::chain::ChainGrinder;

    #[test]
    fn addresses_start_with_t1() {
        let g = ZcashGrinder;
        for _ in 0..32 {
            let (addr, attempt) = g.grind_attempt();
            assert!(addr.starts_with("t1"), "got {addr}");
            assert_eq!(g.finalize(attempt).address, addr);
        }
    }

    #[test]
    fn prefix_pattern_injects_t1() {
        let g = ZcashGrinder;
        let p = g
            .build_pattern(Some("Love"), None, None, false)
            .expect("pattern");
        assert!(p.prefix.starts_with("t1"), "prefix={}", p.prefix);
    }
}
