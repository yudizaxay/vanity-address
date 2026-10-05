use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use secp256k1::SecretKey;

use super::base58_odds::Base58Layout;
use super::util::{
    build_base58_pattern, grind_secp256k1, matches_pattern, p2pkh_address, secret_from_attempt,
    BASE58_ALPHABET,
};

#[derive(Clone)]
pub struct BitcoinLikeGrinder {
    pub id: &'static str,
    pub display_name: &'static str,
    pub version_byte: u8,
    /// First character every address gets from `version_byte`.
    pub lead: &'static str,
    pub wallet_hint: &'static str,
    pub pattern_hint: &'static str,
}

impl BitcoinLikeGrinder {
    pub fn bitcoin() -> Self {
        Self {
            id: "btc",
            display_name: "Bitcoin (P2PKH)",
            version_byte: 0x00,
            lead: "1",
            wallet_hint: "Electrum / Sparrow / hardware wallet WIF import",
            pattern_hint: "Base58 (no 0, O, I, l). Addresses always start with 1 (added for you).",
        }
    }

    pub fn litecoin() -> Self {
        Self {
            id: "ltc",
            display_name: "Litecoin (P2PKH)",
            version_byte: 0x30,
            lead: "L",
            wallet_hint: "Litecoin Core / compatible wallets",
            pattern_hint: "Base58 (no 0, O, I, l). Addresses always start with L (added for you).",
        }
    }

    pub fn dogecoin() -> Self {
        Self {
            id: "doge",
            display_name: "Dogecoin",
            version_byte: 0x1e,
            lead: "D",
            wallet_hint: "Dogecoin Core / compatible wallets",
            pattern_hint: "Base58 (no 0, O, I, l). Addresses always start with D (added for you).",
        }
    }

    pub fn dash() -> Self {
        Self {
            id: "dash",
            display_name: "Dash (P2PKH)",
            version_byte: 0x4c,
            lead: "X",
            wallet_hint: "Dash Core / compatible wallets",
            pattern_hint: "Base58 (no 0, O, I, l). Addresses always start with X (added for you).",
        }
    }

    /// Version byte + 20-byte hash160 + 4-byte checksum.
    fn layout(&self) -> Base58Layout<'_> {
        Base58Layout {
            version: std::slice::from_ref(&self.version_byte),
            random_len: 24,
            alphabet: BASE58_ALPHABET,
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
        let secret_key = SecretKey::from_slice(&secret_bytes).expect("valid secp256k1 secret");
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
        contains: Option<&str>,
        exact: bool,
    ) -> Result<Pattern, String> {
        let mut pattern =
            build_base58_pattern(prefix, suffix, contains, exact, BASE58_ALPHABET, 34)?;
        self.layout()
            .apply_lead(self.display_name, &mut pattern, self.lead)?;
        Ok(pattern)
    }

    fn expected_attempts(&self, pattern: &Pattern) -> f64 {
        self.layout().expected_attempts(pattern)
    }

    fn matches(&self, address: &str, pattern: &Pattern) -> bool {
        matches_pattern(address, pattern, false)
    }

    fn supports_exact_case(&self) -> bool {
        true
    }

    fn pattern_hint(&self) -> &'static str {
        self.pattern_hint
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

    #[test]
    fn dash_address_starts_with_x() {
        let secret = random_secp256k1_secret();
        let addr = p2pkh_address(&secret, BitcoinLikeGrinder::dash().version_byte);
        assert!(addr.starts_with('X'));
    }
}
