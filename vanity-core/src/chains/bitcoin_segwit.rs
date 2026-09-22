use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use bech32::{hrp, segwit};
use rand::rngs::OsRng;
use secp256k1::{Keypair, Scalar, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};

use super::util::{
    build_base58_pattern, expected_from_pattern, hash160, matches_pattern, secret_from_attempt,
    BECH32_CHARSET,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BtcAddressType {
    /// Native SegWit P2WPKH — `bc1q…`
    SegWit,
    /// Taproot P2TR — `bc1p…` (BIP341 key-path, empty script tree)
    Taproot,
}

#[derive(Clone)]
pub struct BitcoinSegwitGrinder {
    pub address_type: BtcAddressType,
}

impl BitcoinSegwitGrinder {
    pub fn segwit() -> Self {
        Self {
            address_type: BtcAddressType::SegWit,
        }
    }

    pub fn taproot() -> Self {
        Self {
            address_type: BtcAddressType::Taproot,
        }
    }

    fn derive_segwit(secret: &SecretKey) -> String {
        let secp = Secp256k1::new();
        let pubkey = secret.public_key(&secp).serialize();
        let program = hash160(&pubkey);
        segwit::encode_v0(hrp::BC, &program).expect("valid p2wpkh")
    }

    /// BIP341 tagged hash: SHA256(SHA256(tag) || SHA256(tag) || msg)
    fn tagged_hash(tag: &[u8], msg: &[u8]) -> [u8; 32] {
        let tag_hash = Sha256::digest(tag);
        let mut hasher = Sha256::new();
        hasher.update(tag_hash);
        hasher.update(tag_hash);
        hasher.update(msg);
        hasher.finalize().into()
    }

    /// Key-path-only Taproot: tweak internal key with empty merkle root.
    /// Returns (address, tweaked secret bytes for wallet import).
    fn derive_taproot(secret: &SecretKey) -> (String, [u8; 32]) {
        let secp = Secp256k1::new();
        let keypair = Keypair::from_secret_key(&secp, secret);
        let (xonly, _parity) = keypair.x_only_public_key();
        let tweak_hash = Self::tagged_hash(b"TapTweak", &xonly.serialize());
        let tweak = Scalar::from_be_bytes(tweak_hash).unwrap_or(Scalar::ZERO);
        let tweaked = keypair
            .add_xonly_tweak(&secp, &tweak)
            .expect("taproot tweak");
        let (out_xonly, _) = tweaked.x_only_public_key();
        let address =
            segwit::encode_v1(hrp::BC, &out_xonly.serialize()).expect("valid p2tr address");
        (address, tweaked.secret_key().secret_bytes())
    }

    fn derive(&self, secret: &SecretKey) -> String {
        match self.address_type {
            BtcAddressType::SegWit => Self::derive_segwit(secret),
            BtcAddressType::Taproot => Self::derive_taproot(secret).0,
        }
    }
}

impl ChainGrinder for BitcoinSegwitGrinder {
    fn id(&self) -> &'static str {
        match self.address_type {
            BtcAddressType::SegWit => "btc-segwit",
            BtcAddressType::Taproot => "btc-taproot",
        }
    }

    fn display_name(&self) -> &'static str {
        match self.address_type {
            BtcAddressType::SegWit => "Bitcoin SegWit (P2WPKH)",
            BtcAddressType::Taproot => "Bitcoin Taproot (P2TR)",
        }
    }

    fn grind_attempt(&self) -> (String, GrindAttempt) {
        let secret = SecretKey::new(&mut OsRng);
        let address = self.derive(&secret);
        (address, GrindAttempt::Secret32(secret.secret_bytes()))
    }

    fn finalize(&self, attempt: GrindAttempt) -> KeypairResult {
        let secret_bytes = secret_from_attempt(attempt);
        let secret_key = SecretKey::from_slice(&secret_bytes).expect("valid secp256k1 secret");

        let address = self.derive(&secret_key);

        let hint = match self.address_type {
            BtcAddressType::SegWit => "Sparrow / Electrum / hardware wallet (native SegWit)",
            BtcAddressType::Taproot => {
                "Sparrow / Electrum Taproot — internal key (empty script tree BIP341)"
            }
        };

        KeypairResult {
            address,
            exports: vec![
                KeyExport {
                    label: "Private Key (hex)".into(),
                    value: hex::encode(secret_bytes),
                    hint: Some(hint.into()),
                },
                KeyExport {
                    label: "Private Key (WIF)".into(),
                    value: wif_encode(secret_bytes),
                    hint: Some("Compressed WIF".into()),
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
        // Bech32 charset; allow matching without forcing bc1q/bc1p in the pattern.
        let mut pattern =
            build_base58_pattern(prefix, suffix, contains, exact, BECH32_CHARSET, 62)?;
        let hrp = match self.address_type {
            BtcAddressType::SegWit => "bc1q",
            BtcAddressType::Taproot => "bc1p",
        };
        if pattern.has_prefix() && !pattern.prefix.starts_with(hrp) {
            // If user typed a data-only prefix, prepend the fixed witness HRP.
            if !pattern.prefix.starts_with("bc1") {
                pattern.prefix = format!("{hrp}{}", pattern.prefix);
                pattern.prefix_match = if pattern.ignore_case {
                    pattern.prefix.to_ascii_lowercase()
                } else {
                    pattern.prefix.clone()
                };
            }
        }
        Ok(pattern)
    }

    fn expected_attempts(&self, pattern: &Pattern) -> f64 {
        expected_from_pattern(pattern, |p| {
            let data = p
                .strip_prefix("bc1q")
                .or_else(|| p.strip_prefix("bc1p"))
                .or_else(|| p.strip_prefix("bc1"))
                .unwrap_or(p);
            32f64.powi(data.chars().filter(|c| *c != '*').count() as i32)
        })
    }

    fn matches(&self, address: &str, pattern: &Pattern) -> bool {
        matches_pattern(address, pattern, true)
    }

    fn supports_exact_case(&self) -> bool {
        false
    }

    fn pattern_hint(&self) -> &'static str {
        match self.address_type {
            BtcAddressType::SegWit => "Bech32 chars after bc1q (or full address). Lowercase only.",
            BtcAddressType::Taproot => "Bech32 chars after bc1p (or full address). Lowercase only.",
        }
    }
}

fn wif_encode(secret: [u8; 32]) -> String {
    let mut payload = vec![0x80];
    payload.extend_from_slice(&secret);
    payload.push(0x01);
    super::util::base58_check_encode_raw(&payload)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chain::ChainGrinder;

    #[test]
    fn segwit_starts_with_bc1q() {
        let g = BitcoinSegwitGrinder::segwit();
        let (addr, _) = g.grind_attempt();
        assert!(addr.starts_with("bc1q"), "got {addr}");
    }

    #[test]
    fn taproot_starts_with_bc1p() {
        let g = BitcoinSegwitGrinder::taproot();
        let (addr, _) = g.grind_attempt();
        assert!(addr.starts_with("bc1p"), "got {addr}");
    }

    #[test]
    fn segwit_roundtrip_finalize() {
        let g = BitcoinSegwitGrinder::segwit();
        let (addr, attempt) = g.grind_attempt();
        let kp = g.finalize(attempt);
        assert_eq!(kp.address, addr);
    }
}
