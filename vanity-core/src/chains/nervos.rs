//! Nervos CKB short address (`ckb1qyq…`) — secp256k1 + blake160.
//!
//! Deprecated short payload `0x01 | 0x00 | blake160(pk)` is still widely used
//! (Neuron / many explorers). HRP `ckb`, Bech32 (BIP-173).

use bech32::{encode, Bech32, Hrp};
use blake2b_simd::Params;
use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use secp256k1::{Secp256k1, SecretKey};

use super::util::{
    address_start_factor, apply_address_start, bech32_combinations, build_base58_pattern,
    expected_from_pattern, grind_secp256k1, matches_pattern, secret_from_attempt, BECH32_CHARSET,
};

/// Short format `0x01` + code_hash_index `0x00` → payload starts `qyq…` after `ckb1`.
const CKB_NEXT: &[&str] = &["q", "y", "q"];

#[derive(Clone, Default)]
pub struct NervosGrinder;

fn ckb_blake160(data: &[u8]) -> [u8; 20] {
    let hash = Params::new()
        .hash_length(32)
        .personal(b"ckb-default-hash")
        .to_state()
        .update(data)
        .finalize();
    let mut out = [0u8; 20];
    out.copy_from_slice(&hash.as_bytes()[..20]);
    out
}

impl NervosGrinder {
    fn derive(secret: &SecretKey) -> String {
        let secp = Secp256k1::new();
        let args = ckb_blake160(&secret.public_key(&secp).serialize());
        let mut payload = Vec::with_capacity(22);
        payload.push(0x01);
        payload.push(0x00);
        payload.extend_from_slice(&args);
        let hrp = Hrp::parse("ckb").expect("hrp");
        encode::<Bech32>(hrp, &payload).expect("bech32")
    }
}

impl ChainGrinder for NervosGrinder {
    fn id(&self) -> &'static str {
        "ckb"
    }

    fn display_name(&self) -> &'static str {
        "Nervos CKB (ckb1qyq…)"
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
                hint: Some("Neuron / CKB wallet import".into()),
            }],
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
            build_base58_pattern(prefix, suffix, contains, exact, BECH32_CHARSET, 90)?;
        apply_address_start(&mut pattern, "Nervos CKB", "ckb1", CKB_NEXT)?;
        Ok(pattern)
    }

    fn expected_attempts(&self, pattern: &Pattern) -> f64 {
        expected_from_pattern(pattern, bech32_combinations)
            * address_start_factor(pattern, CKB_NEXT, 32.0)
    }

    fn matches(&self, address: &str, pattern: &Pattern) -> bool {
        matches_pattern(address, pattern, true)
    }

    fn supports_exact_case(&self) -> bool {
        false
    }

    fn pattern_hint(&self) -> &'static str {
        "Bech32. Short secp addresses look like ckb1qyq… — start after the fixed head."
    }
}

#[cfg(test)]
mod tests {
    use super::{ckb_blake160, NervosGrinder};
    use crate::chain::ChainGrinder;

    #[test]
    fn blake160_known_vector() {
        // nervosnetwork/rfcs#362
        let pk = hex::decode(
            "038ab113db7fc7af23b104a839c5c2e33e4b6b791d0589c3030f1eb7b819c17b82",
        )
        .unwrap();
        assert_eq!(
            hex::encode(ckb_blake160(&pk)),
            "adf33350760d7d939935c48751886ad55506c741"
        );
    }

    #[test]
    fn short_address_known_vector() {
        // CipherWang/ckb-address-demo — args → mainnet short address
        let args = hex::decode("b39bbc0b3673c7d36450bc14cfcdad2d559c6c64").unwrap();
        let mut payload = vec![0x01, 0x00];
        payload.extend_from_slice(&args);
        let hrp = bech32::Hrp::parse("ckb").unwrap();
        let addr = bech32::encode::<bech32::Bech32>(hrp, &payload).unwrap();
        assert_eq!(addr, "ckb1qyqt8xaupvm8837nv3gtc9x0ekkj64vud3jqfwyw5v");
    }

    #[test]
    fn grind_ckb_prefix() {
        let g = NervosGrinder;
        let (addr, attempt) = g.grind_attempt();
        assert!(addr.starts_with("ckb1qyq"), "got {addr}");
        assert_eq!(g.finalize(attempt).address, addr);
    }
}
