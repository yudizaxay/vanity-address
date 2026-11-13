//! Verify that a private key (or CREATE2 salt) derives the claimed address.

use crate::chain::ChainGrinder;
use crate::chains::Chain;
use hex::FromHex;

/// Returns `Ok(())` when `key` derives `address` on `chain`.
///
/// Accepted key encodings (tried in order):
/// - hex (with or without `0x`) — 32-byte secret / salt
/// - base58 (Solana Phantom / Solflare 64-byte keypair, or raw 32-byte secret)
pub fn verify_address(chain_id: &str, address: &str, key: &str) -> Result<(), String> {
    let chain = Chain::from_id(chain_id)?;
    let secret = parse_secret(key)?;
    let attempt = crate::chain::GrindAttempt::Secret32(secret);
    let derived = chain.finalize(attempt);
    // CashAddr may be given with or without the `bitcoincash:` HRP.
    let strip = |a: &str| -> String {
        let a = a.trim();
        if chain_id.eq_ignore_ascii_case("bch") {
            let lower = a.to_ascii_lowercase();
            lower
                .strip_prefix("bitcoincash:")
                .unwrap_or(&lower)
                .to_string()
        } else {
            a.to_string()
        }
    };
    if strip(&derived.address).eq_ignore_ascii_case(&strip(address)) {
        Ok(())
    } else {
        Err(format!(
            "key does not derive address\n  claimed:  {address}\n  derived:  {}",
            derived.address
        ))
    }
}

fn parse_secret(key: &str) -> Result<[u8; 32], String> {
    let trimmed = key.trim();
    if let Some(bytes) = try_hex(trimmed) {
        return bytes;
    }
    // base58 — Solana keypair (64 bytes) or 32-byte secret
    let decoded = bs58::decode(trimmed)
        .into_vec()
        .map_err(|e| format!("invalid key encoding (not hex or base58): {e}"))?;
    match decoded.len() {
        32 => {
            let mut out = [0u8; 32];
            out.copy_from_slice(&decoded);
            Ok(out)
        }
        64 => {
            // Solana keypair JSON/base58: first 32 = secret
            let mut out = [0u8; 32];
            out.copy_from_slice(&decoded[..32]);
            Ok(out)
        }
        n => Err(format!(
            "decoded key is {n} bytes — expected 32 (secret) or 64 (Solana keypair)"
        )),
    }
}

fn try_hex(s: &str) -> Option<Result<[u8; 32], String>> {
    let stripped = s.strip_prefix("0x").unwrap_or(s);
    if !stripped.chars().all(|c| c.is_ascii_hexdigit()) || stripped.len() != 64 {
        return None;
    }
    Some(<[u8; 32]>::from_hex(stripped).map_err(|e| format!("invalid hex key: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chain::ChainGrinder;
    use crate::chains::Chain;

    #[test]
    fn verify_evm_roundtrip() {
        let chain = Chain::from_id("evm").unwrap();
        let (addr, attempt) = chain.grind_attempt();
        let kp = chain.finalize(attempt);
        let hex_key = &kp.exports[0].value;
        assert!(verify_address("evm", &addr, hex_key).is_ok());
        assert!(verify_address("evm", &addr, &format!("0x{hex_key}")).is_ok());
        assert!(verify_address("evm", "0xdeadface", hex_key).is_err());
    }

    #[test]
    fn verify_new_chains_roundtrip_and_reject_tampered() {
        for id in ["zec", "bch", "stx", "dcr", "ckb"] {
            let chain = Chain::from_id(id).unwrap();
            let (addr, attempt) = chain.grind_attempt();
            let kp = chain.finalize(attempt);
            let key = &kp.exports[0].value;
            assert!(verify_address(id, &addr, key).is_ok(), "{id} roundtrip");
            // Change the last character to a *different letter* (compare is
            // case-insensitive, so `Q` → `q` would still match).
            let mut bad = addr.clone();
            let last = bad.pop().unwrap();
            bad.push(if last.eq_ignore_ascii_case(&'q') {
                'p'
            } else {
                'q'
            });
            assert!(verify_address(id, &bad, key).is_err(), "{id} tampered");
        }
    }

    #[test]
    fn verify_bch_accepts_bare_and_prefixed() {
        let chain = Chain::from_id("bch").unwrap();
        let (addr, attempt) = chain.grind_attempt();
        let key = chain.finalize(attempt).exports[0].value.clone();
        let bare = addr.strip_prefix("bitcoincash:").unwrap();
        assert!(verify_address("bch", bare, &key).is_ok());
        assert!(verify_address("bch", &addr, &key).is_ok());
    }
}
