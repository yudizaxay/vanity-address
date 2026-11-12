//! Bitcoin Cash CashAddr P2PKH (`bitcoincash:q…` / bare `q…`).
//!
//! Same secp256k1 + hash160 as Bitcoin; encoding is CashAddr (not legacy Base58).

use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use secp256k1::{Secp256k1, SecretKey};

use super::util::{
    grind_secp256k1, hash160, matches_pattern, secret_from_attempt, validate_chars,
};

const PREFIX: &str = "bitcoincash";
const CHARSET: &[u8] = b"qpzry9x8gf2tvdw0s3jn54khce6mua7l";
/// CashAddr alphabet (same as bech32 charset) for pattern validation.
const CASHADDR_ALPHABET: &str = "qpzry9x8gf2tvdw0s3jn54khce6mua7l";

#[derive(Clone, Default)]
pub struct BitcoinCashGrinder;

impl BitcoinCashGrinder {
    fn derive(secret: &SecretKey) -> String {
        let secp = Secp256k1::new();
        let hash = hash160(&secret.public_key(&secp).serialize());
        encode_cashaddr(PREFIX, 0x00, &hash)
    }

    fn wif_encode(secret: [u8; 32]) -> String {
        let mut payload = vec![0x80];
        payload.extend_from_slice(&secret);
        payload.push(0x01);
        super::util::base58_check_encode_raw(&payload)
    }

    /// Normalize user pattern to operate on the bare `q…` body (strip HRP).
    fn bare(address: &str) -> &str {
        address
            .strip_prefix("bitcoincash:")
            .or_else(|| address.strip_prefix("BITCOINCASH:"))
            .unwrap_or(address)
    }
}

impl ChainGrinder for BitcoinCashGrinder {
    fn id(&self) -> &'static str {
        "bch"
    }

    fn display_name(&self) -> &'static str {
        "Bitcoin Cash (CashAddr q…)"
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
                    hint: Some("Electron Cash / BCH wallets".into()),
                },
                KeyExport {
                    label: "Private Key (WIF)".into(),
                    value: Self::wif_encode(secret_bytes),
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
        // Patterns match the bare CashAddr body (always starts with q for P2PKH).
        let clean = |raw: Option<&str>, label: &str| -> Result<Option<String>, String> {
            match raw {
                None => Ok(None),
                Some(s) => {
                    let s = s
                        .strip_prefix("bitcoincash:")
                        .or_else(|| s.strip_prefix("BITCOINCASH:"))
                        .unwrap_or(s);
                    Ok(Some(validate_chars(
                        label,
                        s,
                        CASHADDR_ALPHABET,
                    )?))
                }
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
        // P2PKH CashAddr body always starts with q
        if pattern.has_prefix() && !pattern.prefix.to_ascii_lowercase().starts_with('q') {
            pattern.prefix = format!("q{}", pattern.prefix);
            if !exact {
                pattern.prefix_match = pattern.prefix.to_ascii_lowercase();
            } else {
                pattern.prefix_match = pattern.prefix.clone();
            }
        }
        if pattern.has_prefix() {
            let lead = pattern.prefix.chars().next().unwrap_or('?');
            if lead.to_ascii_lowercase() != 'q' {
                return Err("Bitcoin Cash CashAddr P2PKH always starts with 'q'".into());
            }
            pattern.set_fixed_prefix("q");
        }
        Ok(pattern)
    }

    fn expected_attempts(&self, pattern: &Pattern) -> f64 {
        // CashAddr charset is 32 symbols; lead `q` is fixed and excluded from the count.
        let mut attempts = 1.0_f64;
        let count = |s: &str, skip_leading_q: bool| {
            let mut n = 0.0_f64;
            let mut chars = s.chars().peekable();
            if skip_leading_q {
                if let Some(c) = chars.peek() {
                    if c.to_ascii_lowercase() == 'q' {
                        chars.next();
                    }
                }
            }
            for c in chars {
                if c != '*' {
                    n += 1.0;
                }
            }
            n
        };
        attempts *= 32.0_f64.powf(count(&pattern.prefix, true));
        attempts *= 32.0_f64.powf(count(&pattern.suffix, false));
        attempts *= 32.0_f64.powf(count(&pattern.contains, false));
        attempts.max(1.0)
    }

    fn matches(&self, address: &str, pattern: &Pattern) -> bool {
        let bare = Self::bare(address);
        matches_pattern(bare, pattern, false)
    }

    fn supports_exact_case(&self) -> bool {
        false
    }

    fn pattern_hint(&self) -> &'static str {
        "CashAddr base32 (q…p…z…). P2PKH always starts with q; bitcoincash: prefix optional."
    }
}

fn encode_cashaddr(prefix: &str, version: u8, hash20: &[u8; 20]) -> String {
    let mut payload = Vec::with_capacity(21);
    payload.push(version);
    payload.extend_from_slice(hash20);
    let data5 = convert_bits(&payload, 8, 5, true);
    let mut values: Vec<u8> = prefix
        .bytes()
        .map(|b| b & 0x1f)
        .chain(std::iter::once(0u8))
        .chain(data5.iter().copied())
        .chain(std::iter::repeat(0u8).take(8))
        .collect();
    let mod_ = polymod(&values) ^ 1;
    values.truncate(values.len() - 8);
    for i in 0..8 {
        values.push(((mod_ >> (5 * (7 - i))) & 0x1f) as u8);
    }
    // strip prefix expand — values currently: prefix_expand + 0 + data5 + checksum
    let body_start = prefix.len() + 1;
    let body: String = values[body_start..]
        .iter()
        .map(|&v| CHARSET[v as usize] as char)
        .collect();
    format!("{prefix}:{body}")
}

fn convert_bits(data: &[u8], from: u32, to: u32, pad: bool) -> Vec<u8> {
    let mut acc = 0u32;
    let mut bits = 0u32;
    let mut ret = Vec::new();
    let maxv = (1u32 << to) - 1;
    for &b in data {
        acc = (acc << from) | u32::from(b);
        bits += from;
        while bits >= to {
            bits -= to;
            ret.push(((acc >> bits) & maxv) as u8);
        }
    }
    if pad && bits > 0 {
        ret.push(((acc << (to - bits)) & maxv) as u8);
    }
    ret
}

fn polymod(values: &[u8]) -> u64 {
    const GEN: [u64; 5] = [
        0x98f2_bc8e_61,
        0x79b7_6d99_e2,
        0xf33e_5fb3_c4,
        0xae2e_abe2_a8,
        0x1e4f_43e4_70,
    ];
    let mut c = 1u64;
    for &v in values {
        let c0 = c >> 35;
        c = ((c & 0x07_ffff_ffff) << 5) ^ u64::from(v);
        for (i, g) in GEN.iter().enumerate() {
            if (c0 >> i) & 1 != 0 {
                c ^= g;
            }
        }
    }
    c
}

#[cfg(test)]
mod tests {
    use super::{encode_cashaddr, BitcoinCashGrinder};
    use crate::chain::ChainGrinder;

    #[test]
    fn known_vector_cashaddr() {
        // From CashAddr docs: hash 211b74ca4686f81efda5641767fc84ef16dafe0b
        let hash = hex::decode("211b74ca4686f81efda5641767fc84ef16dafe0b").unwrap();
        let mut h = [0u8; 20];
        h.copy_from_slice(&hash);
        let addr = encode_cashaddr("bitcoincash", 0x00, &h);
        assert_eq!(
            addr,
            "bitcoincash:qqs3kax2g6r0s8ha54jpwelusnh3dkh7pvu23rzrru"
        );
    }

    #[test]
    fn grind_starts_with_q() {
        let g = BitcoinCashGrinder;
        let (addr, _) = g.grind_attempt();
        assert!(addr.starts_with("bitcoincash:q"), "got {addr}");
    }

    #[test]
    fn prefix_gets_q_lead() {
        let g = BitcoinCashGrinder;
        let p = g
            .build_pattern(Some("test"), None, None, false)
            .expect("ok");
        assert!(p.prefix.starts_with('q') || p.prefix.starts_with('Q'));
    }
}
