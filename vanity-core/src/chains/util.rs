use crate::chain::GrindAttempt;
use crate::pattern::{matches_both, Pattern};
use rand::rngs::OsRng;
use ripemd::Ripemd160;
use secp256k1::{Secp256k1, SecretKey};
use sha2::{Digest, Sha256};
use solana_sdk::signature::{Keypair, SeedDerivable};

pub const BASE58_ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub const RIPPLE_ALPHABET: &str = "rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz";

/// Bech32 data charset (cosmos1… addresses).
pub const BECH32_CHARSET: &str = "qpzry9x8gf2tvdw0s3jn54khce6mua7l";

pub fn hash160(data: &[u8]) -> [u8; 20] {
    let sha = Sha256::digest(data);
    let rip = Ripemd160::digest(sha);
    let mut out = [0u8; 20];
    out.copy_from_slice(&rip);
    out
}

pub fn random_secp256k1_secret() -> SecretKey {
    SecretKey::new(&mut OsRng)
}

pub fn grind_secp256k1<F>(derive: F) -> (String, GrindAttempt)
where
    F: Fn(&SecretKey) -> String,
{
    let secret = random_secp256k1_secret();
    let address = derive(&secret);
    (address, GrindAttempt::Secret32(secret.secret_bytes()))
}

pub fn base58_check_encode(version: u8, payload: &[u8]) -> String {
    let mut data = Vec::with_capacity(1 + payload.len() + 4);
    data.push(version);
    data.extend_from_slice(payload);
    let checksum = &Sha256::digest(Sha256::digest(&data))[..4];
    data.extend_from_slice(checksum);
    bs58::encode(data).into_string()
}

pub fn base58_check_encode_raw(payload: &[u8]) -> String {
    let checksum = &Sha256::digest(Sha256::digest(payload))[..4];
    let mut data = payload.to_vec();
    data.extend_from_slice(checksum);
    bs58::encode(data).into_string()
}

pub fn encode_base58_with_alphabet(data: &[u8], alphabet: &str) -> String {
    if data.is_empty() {
        return String::new();
    }

    let mut digits = vec![0u8];
    for &byte in data {
        let mut carry = byte as u32;
        for digit in &mut digits {
            carry += (*digit as u32) * 256;
            *digit = (carry % 58) as u8;
            carry /= 58;
        }
        while carry > 0 {
            digits.push((carry % 58) as u8);
            carry /= 58;
        }
    }

    let mut encoded = String::new();
    for &byte in data {
        if byte == 0 {
            encoded.push(alphabet.chars().next().unwrap());
        } else {
            break;
        }
    }
    for &digit in digits.iter().rev() {
        let ch = alphabet.chars().nth(digit as usize).unwrap_or_else(|| {
            panic!(
                "base58 digit {digit} out of range for alphabet len {}",
                alphabet.len()
            )
        });
        encoded.push(ch);
    }
    encoded
}

pub fn p2pkh_address(secret: &SecretKey, version: u8) -> String {
    let secp = Secp256k1::new();
    let pubkey = secret.public_key(&secp).serialize();
    let hash = hash160(&pubkey);
    base58_check_encode(version, &hash)
}

pub fn validate_chars(label: &str, pattern: &str, allowed: &str) -> Result<String, String> {
    if pattern.is_empty() {
        return Err(format!("'{label}' cannot be empty"));
    }
    for c in pattern.chars() {
        if !allowed.contains(c) {
            return Err(format!("'{label}' contains '{c}' — allowed: {allowed}"));
        }
    }
    Ok(pattern.to_string())
}

pub fn build_base58_pattern(
    prefix: Option<&str>,
    suffix: Option<&str>,
    exact: bool,
    allowed: &str,
    max_len: usize,
) -> Result<Pattern, String> {
    let prefix_raw = prefix.unwrap_or("").to_string();
    let suffix_raw = suffix.unwrap_or("").to_string();

    if prefix_raw.is_empty() && suffix_raw.is_empty() {
        return Err("Provide at least one of --prefix or --suffix".into());
    }

    let prefix = if prefix_raw.is_empty() {
        String::new()
    } else {
        let validated = validate_chars("prefix", &prefix_raw, allowed)?;
        if validated.len() > max_len {
            return Err(format!("prefix is too long (max {max_len} characters)"));
        }
        validated
    };

    let suffix = if suffix_raw.is_empty() {
        String::new()
    } else {
        let validated = validate_chars("suffix", &suffix_raw, allowed)?;
        if validated.len() > max_len {
            return Err(format!("suffix is too long (max {max_len} characters)"));
        }
        validated
    };

    if !prefix.is_empty() && !suffix.is_empty() && prefix.len() + suffix.len() > max_len {
        return Err(format!(
            "prefix + suffix length ({}) exceeds {max_len} characters",
            prefix.len() + suffix.len()
        ));
    }

    let (prefix_match, suffix_match) = if exact {
        (prefix.clone(), suffix.clone())
    } else {
        (prefix.to_ascii_lowercase(), suffix.to_ascii_lowercase())
    };

    Ok(Pattern {
        prefix,
        suffix,
        prefix_match,
        suffix_match,
        ignore_case: !exact,
    })
}

pub fn build_hex_pattern(
    prefix: Option<&str>,
    suffix: Option<&str>,
    with_0x_prefix: bool,
    max_hex_len: usize,
) -> Result<Pattern, String> {
    let prefix_raw = prefix.unwrap_or("").to_string();
    let suffix_raw = suffix.unwrap_or("").to_string();

    if prefix_raw.is_empty() && suffix_raw.is_empty() {
        return Err("Provide at least one of --prefix or --suffix".into());
    }

    let normalize = |label: &str, input: &str| -> Result<String, String> {
        let stripped = input.strip_prefix("0x").unwrap_or(input);
        let normalized = stripped.to_ascii_lowercase();
        if normalized.is_empty() {
            return Err(format!("'{label}' cannot be empty"));
        }
        for c in normalized.chars() {
            if !c.is_ascii_hexdigit() {
                return Err(format!("'{label}' contains '{c}' — must be hex (0-9, a-f)"));
            }
        }
        Ok(normalized)
    };

    let prefix = if prefix_raw.is_empty() {
        String::new()
    } else {
        let normalized = normalize("prefix", &prefix_raw)?;
        if normalized.len() > max_hex_len {
            return Err(format!("prefix is too long (max {max_hex_len} hex chars)"));
        }
        if with_0x_prefix {
            format!("0x{normalized}")
        } else {
            normalized
        }
    };

    let suffix = if suffix_raw.is_empty() {
        String::new()
    } else {
        let normalized = normalize("suffix", &suffix_raw)?;
        if normalized.len() > max_hex_len {
            return Err(format!("suffix is too long (max {max_hex_len} hex chars)"));
        }
        normalized
    };

    let prefix_hex_len = prefix.strip_prefix("0x").unwrap_or(&prefix).len();
    if !prefix.is_empty() && !suffix.is_empty() && prefix_hex_len + suffix.len() > max_hex_len {
        return Err(format!(
            "prefix + suffix length ({}) exceeds {max_hex_len} hex characters",
            prefix_hex_len + suffix.len()
        ));
    }

    Ok(Pattern {
        prefix: prefix.clone(),
        suffix: suffix.clone(),
        prefix_match: prefix,
        suffix_match: suffix,
        ignore_case: true,
    })
}

pub fn base58_combinations(pattern: &str) -> f64 {
    58f64.powi(pattern.len() as i32)
}

pub fn hex_combinations(pattern: &str) -> f64 {
    let hex_len = pattern.strip_prefix("0x").unwrap_or(pattern).len();
    16f64.powi(hex_len as i32)
}

pub fn bech32_combinations(pattern: &str) -> f64 {
    32f64.powi(pattern.len() as i32)
}

pub fn base32_combinations(pattern: &str) -> f64 {
    32f64.powi(pattern.len() as i32)
}

pub fn base64url_combinations(pattern: &str) -> f64 {
    64f64.powi(pattern.len() as i32)
}

/// RFC 4648 base32 alphabet (Algorand / Filecoin / ICP).
pub const BASE32_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
pub const BASE32_ALPHABET_LOWER: &str = "abcdefghijklmnopqrstuvwxyz234567";

/// TON user-friendly address alphabet (base64url).
pub const BASE64URL_ALPHABET: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

/// DER-encodes a raw 32-byte Ed25519 public key as a SubjectPublicKeyInfo
/// (RFC 8410), used by both ICP principals and Hedera's public key export.
pub fn der_ed25519_spki(pubkey: &[u8; 32]) -> Vec<u8> {
    let mut der = vec![
        0x30, 0x2a, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x03, 0x21, 0x00,
    ];
    der.extend_from_slice(pubkey);
    der
}

pub fn blake2b_var(data: &[u8], out_len: usize) -> Vec<u8> {
    use blake2::digest::{Update, VariableOutput};
    use blake2::Blake2bVar;
    let mut hasher = Blake2bVar::new(out_len).expect("valid blake2b length");
    hasher.update(data);
    let mut out = vec![0u8; out_len];
    hasher
        .finalize_variable(&mut out)
        .expect("blake2b finalize");
    out
}

/// Kaspa's address checksum (a CashAddr-style BCH code over GF(2^5)) — NOT
/// standard BIP-173 bech32, despite using the same 32-symbol charset. Ported
/// from kaspa-addresses (rusty-kaspa) and verified against its own test
/// vectors in `kaspa::tests::kaspa_matches_known_vector`.
fn cashaddr_polymod(values: impl Iterator<Item = u8>) -> u64 {
    let mut c: u64 = 1;
    for d in values {
        let c0 = c >> 35;
        c = ((c & 0x0007_ffff_ffff) << 5) ^ (d as u64);
        if c0 & 0x01 != 0 {
            c ^= 0x98f2bc8e61;
        }
        if c0 & 0x02 != 0 {
            c ^= 0x79b76d99e2;
        }
        if c0 & 0x04 != 0 {
            c ^= 0xf33e5fb3c4;
        }
        if c0 & 0x08 != 0 {
            c ^= 0xae2eabe2a8;
        }
        if c0 & 0x10 != 0 {
            c ^= 0x1e4f43e470;
        }
    }
    c ^ 1
}

/// Regroups 8-bit bytes into 5-bit words, right-padding the final word with
/// zero bits (matches kaspa-addresses' `conv8to5`).
fn conv8to5(payload: &[u8]) -> Vec<u8> {
    let padding = usize::from(!payload.len().is_multiple_of(5));
    let mut out = vec![0u8; payload.len() * 8 / 5 + padding];
    let mut idx = 0;
    let mut buf: u16 = 0;
    let mut bits = 0u32;
    for &byte in payload {
        buf = (buf << 8) | u16::from(byte);
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            out[idx] = (buf >> bits) as u8;
            buf &= (1 << bits) - 1;
            idx += 1;
        }
    }
    if bits > 0 {
        out[idx] = (buf << (5 - bits)) as u8;
    }
    out
}

/// Encodes a Kaspa address body (`version` byte + raw payload, e.g. a
/// 32-byte Schnorr x-only pubkey) for the given prefix ("kaspa" / "kaspatest"),
/// returning just the data+checksum part (caller adds `prefix:`).
pub fn kaspa_address_data(prefix: &str, version: u8, payload: &[u8]) -> String {
    let mut versioned = Vec::with_capacity(1 + payload.len());
    versioned.push(version);
    versioned.extend_from_slice(payload);
    let fivebit_payload = conv8to5(&versioned);

    let fivebit_prefix = prefix.as_bytes().iter().copied().map(|c| c & 0x1f);
    let checksum = cashaddr_polymod(
        fivebit_prefix
            .chain([0u8])
            .chain(fivebit_payload.iter().copied())
            .chain([0u8; 8]),
    );
    let checksum5 = conv8to5(&checksum.to_be_bytes()[3..]);

    let chars: Vec<char> = BECH32_CHARSET.chars().collect();
    fivebit_payload
        .iter()
        .chain(checksum5.iter())
        .map(|&i| chars[i as usize])
        .collect()
}

pub fn crc16_xmodem(data: &[u8]) -> u16 {
    let mut crc: u16 = 0;
    for &b in data {
        crc ^= u16::from(b) << 8;
        for _ in 0..8 {
            if crc & 0x8000 != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}

pub fn expected_from_pattern(pattern: &Pattern, per_char: impl Fn(&str) -> f64) -> f64 {
    let mut combos = 1.0_f64;
    if pattern.has_prefix() {
        combos *= per_char(&pattern.prefix);
    }
    if pattern.has_suffix() {
        combos *= per_char(&pattern.suffix);
    }
    combos
}

pub fn matches_pattern(address: &str, pattern: &Pattern, force_lower: bool) -> bool {
    let addr = if force_lower || pattern.ignore_case {
        address.to_ascii_lowercase()
    } else {
        address.to_string()
    };
    matches_both(
        &addr,
        &pattern.prefix_match,
        &pattern.suffix_match,
        pattern.ignore_case || force_lower,
    )
}

pub fn grind_ed25519<F>(derive: F) -> (String, GrindAttempt)
where
    F: Fn(&Keypair) -> String,
{
    let keypair = Keypair::new();
    let address = derive(&keypair);
    (
        address,
        GrindAttempt::Secret32(secret32_from_keypair(&keypair)),
    )
}

pub fn keypair_from_secret(secret: [u8; 32]) -> Keypair {
    Keypair::from_seed(&secret).expect("valid ed25519 seed")
}

pub fn secret32_from_keypair(keypair: &Keypair) -> [u8; 32] {
    let seed = keypair.secret().to_bytes();
    let mut secret = [0u8; 32];
    secret.copy_from_slice(&seed);
    secret
}

pub fn secret_from_attempt(attempt: GrindAttempt) -> [u8; 32] {
    let GrindAttempt::Secret32(bytes) = attempt else {
        panic!("expected Secret32 grind attempt");
    };
    bytes
}
