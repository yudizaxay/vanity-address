//! Stacks mainnet singlesig addresses (`SP…`) via c32check.
//!
//! hash160(compressed secp256k1 pubkey) + version `0x16` + double-SHA256 checksum,
//! Crockford base32, then prefixed with `S`.

use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use secp256k1::{Secp256k1, SecretKey};
use sha2::{Digest, Sha256};

use super::util::{
    grind_secp256k1, hash160, matches_pattern, secret_from_attempt, validate_chars,
};

/// Mainnet P2PKH / singlesig version (c32check).
const VERSION: u8 = 22; // 0x16
/// Crockford base32 (no I, L, O, U).
const C32: &[u8] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
const C32_ALPHABET: &str = "0123456789ABCDEFGHJKMNPQRSTVWXYZ";

#[derive(Clone, Default)]
pub struct StacksGrinder;

impl StacksGrinder {
    fn derive(secret: &SecretKey) -> String {
        let secp = Secp256k1::new();
        let hash = hash160(&secret.public_key(&secp).serialize());
        c32address(VERSION, &hash)
    }

    fn wif_encode(secret: [u8; 32]) -> String {
        // Stacks often uses hex; WIF still useful for BTC-linked keys.
        let mut payload = vec![0x80];
        payload.extend_from_slice(&secret);
        payload.push(0x01);
        super::util::base58_check_encode_raw(&payload)
    }
}

impl ChainGrinder for StacksGrinder {
    fn id(&self) -> &'static str {
        "stx"
    }

    fn display_name(&self) -> &'static str {
        "Stacks (SP… c32)"
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
                    hint: Some("Hiro / Leather / Xverse import".into()),
                },
                KeyExport {
                    label: "Private Key (WIF)".into(),
                    value: Self::wif_encode(secret_bytes),
                    hint: Some("Same key can derive linked BTC P2PKH".into()),
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
        let clean = |raw: Option<&str>, label: &str| -> Result<Option<String>, String> {
            match raw {
                None => Ok(None),
                Some(s) => Ok(Some(validate_chars(label, &s.to_ascii_uppercase(), C32_ALPHABET)?)),
            }
        };
        let prefix = clean(prefix, "prefix")?;
        let suffix = clean(suffix, "suffix")?;
        let contains = clean(contains, "contains")?;
        if prefix.is_none() && suffix.is_none() && contains.is_none() {
            return Err("Provide at least one of prefix, suffix, or contains".into());
        }
        let prefix = prefix.unwrap_or_default();
        let suffix = suffix.unwrap_or_default();
        let contains = contains.unwrap_or_default();
        let (prefix_match, suffix_match, contains_match) = if exact {
            (prefix.clone(), suffix.clone(), contains.clone())
        } else {
            (
                prefix.to_ascii_lowercase(),
                suffix.to_ascii_lowercase(),
                contains.to_ascii_lowercase(),
            )
        };
        let mut pattern = Pattern {
            prefix,
            suffix,
            contains,
            prefix_match,
            suffix_match,
            contains_match,
            ignore_case: !exact,
            fixed_prefix_len: 0,
        };
        // Mainnet singlesig always starts with SP
        if pattern.has_prefix() {
            let p = pattern.prefix.to_ascii_uppercase();
            pattern.prefix = if p.starts_with("SP") {
                p
            } else if p.starts_with('P') {
                format!("S{p}")
            } else {
                format!("SP{p}")
            };
            pattern.prefix_match = if exact {
                pattern.prefix.clone()
            } else {
                pattern.prefix.to_ascii_lowercase()
            };
            pattern.set_fixed_prefix("SP");
        }
        Ok(pattern)
    }

    fn expected_attempts(&self, pattern: &Pattern) -> f64 {
        let mut attempts = 1.0_f64;
        let count = |s: &str, skip_sp: bool| {
            let mut chars: Vec<char> = s.chars().collect();
            if skip_sp && chars.len() >= 2 {
                let head: String = chars.iter().take(2).collect::<String>().to_ascii_uppercase();
                if head == "SP" {
                    chars.drain(0..2);
                }
            }
            chars.into_iter().filter(|&c| c != '*').count() as f64
        };
        attempts *= 32.0_f64.powf(count(&pattern.prefix, true));
        attempts *= 32.0_f64.powf(count(&pattern.suffix, false));
        attempts *= 32.0_f64.powf(count(&pattern.contains, false));
        attempts.max(1.0)
    }

    fn matches(&self, address: &str, pattern: &Pattern) -> bool {
        // c32 is uppercase; compare case-insensitively unless exact
        matches_pattern(address, pattern, false)
    }

    fn supports_exact_case(&self) -> bool {
        false
    }

    fn pattern_hint(&self) -> &'static str {
        "Crockford base32 (no I L O U). Mainnet singlesig always starts with SP."
    }
}

fn c32address(version: u8, hash160: &[u8; 20]) -> String {
    // c32check: checksum over [version || hash160], encode hash160||checksum,
    // then prepend c32 digit for `version` and a leading `S`.
    let mut for_sum = Vec::with_capacity(21);
    for_sum.push(version);
    for_sum.extend_from_slice(hash160);
    let checksum = &Sha256::digest(Sha256::digest(&for_sum))[..4];
    let mut payload = Vec::with_capacity(24);
    payload.extend_from_slice(hash160);
    payload.extend_from_slice(checksum);
    let ver_char = C32[version as usize] as char;
    format!("S{ver_char}{}", c32encode(&payload))
}

fn c32encode(data: &[u8]) -> String {
    // Big-endian base32 of the byte string (c32check / stacks convention).
    if data.is_empty() {
        return String::new();
    }
    let mut digits = vec![0u8];
    for &byte in data {
        let mut carry = byte as u32;
        for d in &mut digits {
            carry += (*d as u32) * 256;
            *d = (carry % 32) as u8;
            carry /= 32;
        }
        while carry > 0 {
            digits.push((carry % 32) as u8);
            carry /= 32;
        }
    }
    // Leading zero bytes → '0' in c32
    let mut out = String::new();
    for &b in data {
        if b == 0 {
            out.push('0');
        } else {
            break;
        }
    }
    for &d in digits.iter().rev() {
        out.push(C32[d as usize] as char);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{c32address, StacksGrinder};
    use crate::chain::ChainGrinder;

    #[test]
    fn known_vector_sp_address() {
        // From c32check README
        let hash = hex::decode("a46ff88886c2ef9762d970b4d2c63678835bd39d").unwrap();
        let mut h = [0u8; 20];
        h.copy_from_slice(&hash);
        let addr = c32address(22, &h);
        assert_eq!(addr, "SP2J6ZY48GV1EZ5V2V5RB9MP66SW86PYKKNRV9EJ7");
    }

    #[test]
    fn grind_starts_with_sp() {
        let g = StacksGrinder;
        let (addr, attempt) = g.grind_attempt();
        assert!(addr.starts_with("SP"), "got {addr}");
        assert_eq!(g.finalize(attempt).address, addr);
    }
}
