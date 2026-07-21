use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use sha2::{Digest, Sha256};
use solana_sdk::signature::{Keypair, Signer};

use super::util::{
    base64url_combinations, build_base58_pattern, crc16_xmodem, expected_from_pattern,
    grind_ed25519, keypair_from_secret, matches_pattern, secret_from_attempt, BASE64URL_ALPHABET,
};

/// Wallet V4R2 code cell hash (constant) and depth from official BOC.
const V4R2_CODE_HASH: [u8; 32] = [
    0xfe, 0xb5, 0xff, 0x68, 0x20, 0xe2, 0xff, 0x0d, 0x94, 0x83, 0xe7, 0xe0, 0xd6, 0x2c, 0x81, 0x7d,
    0x84, 0x67, 0x89, 0xfb, 0x4a, 0xe5, 0x80, 0xc8, 0x78, 0x86, 0x6d, 0x95, 0x9d, 0xab, 0xd5, 0xc0,
];
const V4R2_CODE_DEPTH: u16 = 7;
const V4R2_WALLET_ID: u32 = 0x29a9a317;
/// Non-bounceable mainnet tag (Tonkeeper-style UQ…).
const TAG_NON_BOUNCEABLE: u8 = 0x51;

#[derive(Clone, Default)]
pub struct TonGrinder;

impl TonGrinder {
    fn data_cell_hash(pubkey: &[u8; 32]) -> [u8; 32] {
        // 321 bits: seqno(32) + wallet_id(32) + pubkey(256) + plugins empty(1)
        let mut bits = [0u8; 41];
        bits[0..4].copy_from_slice(&0u32.to_be_bytes());
        bits[4..8].copy_from_slice(&V4R2_WALLET_ID.to_be_bytes());
        bits[8..40].copy_from_slice(pubkey);
        // last data bit = 0, then completion bit 1 → 0b0100_0000
        bits[40] = 0x40;

        let d1: u8 = 0; // 0 refs
        let d2: u8 = 81; // ceil(321/8)+floor(321/8)
        let mut hasher = Sha256::new();
        hasher.update([d1, d2]);
        hasher.update(bits);
        hasher.finalize().into()
    }

    fn account_id(pubkey: &[u8; 32]) -> [u8; 32] {
        let data_hash = Self::data_cell_hash(pubkey);
        // StateInit cell: 5 bits 00110 + refs [code, data]
        // d1=2, d2=1, data byte 0x34
        let mut hasher = Sha256::new();
        hasher.update([2u8, 1u8, 0x34]);
        hasher.update(V4R2_CODE_DEPTH.to_be_bytes());
        hasher.update(0u16.to_be_bytes()); // data depth
        hasher.update(V4R2_CODE_HASH);
        hasher.update(data_hash);
        hasher.finalize().into()
    }

    fn friendly_address(account_id: &[u8; 32]) -> String {
        let mut buf = [0u8; 34];
        buf[0] = TAG_NON_BOUNCEABLE;
        buf[1] = 0; // workchain 0
        buf[2..].copy_from_slice(account_id);
        let crc = crc16_xmodem(&buf);
        let mut full = [0u8; 36];
        full[..34].copy_from_slice(&buf);
        full[34..].copy_from_slice(&crc.to_be_bytes());
        base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, full)
    }

    fn derive_address(keypair: &Keypair) -> String {
        let pubkey = keypair.pubkey().to_bytes();
        let id = Self::account_id(&pubkey);
        Self::friendly_address(&id)
    }
}

impl ChainGrinder for TonGrinder {
    fn id(&self) -> &'static str {
        "ton"
    }

    fn display_name(&self) -> &'static str {
        "TON (Wallet V4R2)"
    }

    fn grind_attempt(&self) -> (String, GrindAttempt) {
        grind_ed25519(Self::derive_address)
    }

    fn finalize(&self, attempt: GrindAttempt) -> KeypairResult {
        let secret_bytes = secret_from_attempt(attempt);
        let keypair = keypair_from_secret(secret_bytes);
        let address = Self::derive_address(&keypair);

        KeypairResult {
            address,
            exports: vec![KeyExport {
                label: "Private Key (hex)".into(),
                value: hex::encode(secret_bytes),
                hint: Some("Tonkeeper / TON Wallet V4R2 (UQ non-bounceable)".into()),
            }],
        }
    }

    fn build_pattern(
        &self,
        prefix: Option<&str>,
        suffix: Option<&str>,
        exact: bool,
    ) -> Result<Pattern, String> {
        let mut pattern = build_base58_pattern(prefix, suffix, exact, BASE64URL_ALPHABET, 48)?;
        if pattern.has_prefix() && !pattern.prefix.starts_with("UQ") {
            pattern.prefix = format!("UQ{}", pattern.prefix);
            pattern.prefix_match = if pattern.ignore_case {
                pattern.prefix.to_ascii_lowercase()
            } else {
                pattern.prefix.clone()
            };
        }
        Ok(pattern)
    }

    fn expected_attempts(&self, pattern: &Pattern) -> f64 {
        expected_from_pattern(pattern, base64url_combinations)
    }

    fn matches(&self, address: &str, pattern: &Pattern) -> bool {
        matches_pattern(address, pattern, false)
    }

    fn supports_exact_case(&self) -> bool {
        true
    }

    fn pattern_hint(&self) -> &'static str {
        "Base64url — UQ… (Wallet V4R2 non-bounceable)."
    }
}

#[cfg(test)]
mod tests {
    use super::TonGrinder;
    use crate::chain::ChainGrinder;
    use solana_sdk::signature::{Keypair, SeedDerivable};

    #[test]
    fn ton_address_matches_known_vector() {
        let seed = [1u8; 32];
        let keypair = Keypair::from_seed(&seed).unwrap();
        let addr = TonGrinder::derive_address(&keypair);
        assert_eq!(addr, "UQDvr_S6wiD4iy6Y6x2c_8yjv-O2bs4xp9bFiQ0w39evpYZV");
    }

    #[test]
    fn ton_address_starts_with_uq() {
        let g = TonGrinder;
        let (addr, _) = g.grind_attempt();
        assert!(addr.starts_with("UQ"));
        assert_eq!(addr.len(), 48);
    }

    #[test]
    fn build_pattern_always_forces_uq_prefix() {
        let g = TonGrinder;
        // Only "UQ…" addresses are ever derived (non-bounceable tag). A pattern
        // requesting "EQ…" (bounceable) can never match, so it must be normalized
        // to "UQ" instead of grinding forever with zero chance of success.
        let pattern = g.build_pattern(Some("EQabc"), None, false).unwrap();
        assert!(pattern.prefix.starts_with("UQ"));
        assert!(pattern.prefix_match.starts_with("uq"));
    }
}
