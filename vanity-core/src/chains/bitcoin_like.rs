use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use secp256k1::SecretKey;

use super::util::{
    base58_combinations, build_base58_pattern, expected_from_pattern, grind_secp256k1,
    matches_pattern, p2pkh_address, secret_from_attempt, BASE58_ALPHABET,
};

#[derive(Clone)]
pub struct BitcoinLikeGrinder {
    pub id: &'static str,
    pub display_name: &'static str,
    pub version_byte: u8,
    pub wallet_hint: &'static str,
}

impl BitcoinLikeGrinder {
    pub fn bitcoin() -> Self {
        Self {
            id: "btc",
            display_name: "Bitcoin (P2PKH)",
            version_byte: 0x00,
            wallet_hint: "Electrum / Sparrow / hardware wallet WIF import",
        }
    }

    pub fn litecoin() -> Self {
        Self {
            id: "ltc",
            display_name: "Litecoin (P2PKH)",
            version_byte: 0x30,
            wallet_hint: "Litecoin Core / compatible wallets",
        }
    }

    pub fn dogecoin() -> Self {
        Self {
            id: "doge",
            display_name: "Dogecoin",
            version_byte: 0x1e,
            wallet_hint: "Dogecoin Core / compatible wallets",
        }
    }

    fn derive(&self, secret: &SecretKey) -> String {
        p2pkh_address(secret, self.version_byte)
    }
}

impl ChainGrinder for BitcoinLikeGrinder {
    fn id(&self) -> &'static str {
        self.id
    }

    fn display_name(&self) -> &'static str {
        self.display_name
    }

    fn grind_attempt(&self) -> (String, GrindAttempt) {
        grind_secp256k1(|secret| self.derive(secret))
    }

    fn finalize(&self, attempt: GrindAttempt) -> KeypairResult {
        let secret_bytes = secret_from_attempt(attempt);
        let secret_key =
            SecretKey::from_slice(&secret_bytes).expect("valid secp256k1 secret");
        let address = self.derive(&secret_key);

        KeypairResult {
            address,
            exports: vec![
                KeyExport {
                    label: "Private Key (hex)".into(),
                    value: hex::encode(secret_bytes),
                    hint: Some(self.wallet_hint.into()),
                },
                KeyExport {
                    label: "Private Key (WIF)".into(),
                    value: wif_encode(secret_bytes, self.version_byte + 0x80),
                    hint: Some("Standard Bitcoin-family WIF format".into()),
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
        build_base58_pattern(prefix, suffix, exact, BASE58_ALPHABET, 34)
    }

    fn expected_attempts(&self, pattern: &Pattern) -> f64 {
        expected_from_pattern(pattern, base58_combinations)
    }

    fn matches(&self, address: &str, pattern: &Pattern) -> bool {
        matches_pattern(address, pattern, false)
    }

    fn supports_exact_case(&self) -> bool {
        true
    }

    fn pattern_hint(&self) -> &'static str {
        "Base58 characters (no 0, O, I, l)."
    }
}

fn wif_encode(secret: [u8; 32], version: u8) -> String {
    let mut payload = vec![version];
    payload.extend_from_slice(&secret);
    payload.push(0x01); // compressed pubkey flag
    super::util::base58_check_encode_raw(&payload)
}

#[cfg(test)]
mod tests {
    use super::BitcoinLikeGrinder;
    use crate::chains::util::{p2pkh_address, random_secp256k1_secret};

    #[test]
    fn bitcoin_address_starts_with_1() {
        let secret = random_secp256k1_secret();
        let addr = p2pkh_address(&secret, BitcoinLikeGrinder::bitcoin().version_byte);
        assert!(addr.starts_with('1'));
    }

    #[test]
    fn litecoin_address_starts_with_l() {
        let secret = random_secp256k1_secret();
        let addr = p2pkh_address(&secret, BitcoinLikeGrinder::litecoin().version_byte);
        assert!(addr.starts_with('L'));
    }

    #[test]
    fn dogecoin_address_starts_with_d() {
        let secret = random_secp256k1_secret();
        let addr = p2pkh_address(&secret, BitcoinLikeGrinder::dogecoin().version_byte);
        assert!(addr.starts_with('D'));
    }
}
