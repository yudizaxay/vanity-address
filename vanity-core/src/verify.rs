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
    if derived.address.eq_ignore_ascii_case(address.trim()) {
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
        assert!(verify_address("evm", "0xdeadbeef", hex_key).is_err());
    }
}
