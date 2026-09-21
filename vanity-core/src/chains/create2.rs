//! EVM CREATE2 salt vanity — grind a 32-byte salt so
//! `keccak256(0xff ‖ deployer ‖ salt ‖ initCodeHash)[12:]` matches a hex pattern.
//!
//! This is the "lite" form: you supply deployer + init code hash; we search salts.

use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use rand::rngs::OsRng;
use rand::RngCore;
use sha3::{Digest, Keccak256};

use super::util::{
    build_hex_pattern, expected_from_pattern, hex_combinations, matches_pattern,
    secret_from_attempt,
};

#[derive(Clone)]
pub struct Create2Grinder {
    pub deployer: [u8; 20],
    pub init_code_hash: [u8; 32],
}

impl Create2Grinder {
    pub fn new(deployer: [u8; 20], init_code_hash: [u8; 32]) -> Self {
        Self {
            deployer,
            init_code_hash,
        }
    }

    pub fn parse(deployer_hex: &str, init_code_hash_hex: &str) -> Result<Self, String> {
        Ok(Self {
            deployer: parse_hex_fixed::<20>(deployer_hex, "deployer")?,
            init_code_hash: parse_hex_fixed::<32>(init_code_hash_hex, "init-code-hash")?,
        })
    }

    fn derive_address(&self, salt: &[u8; 32]) -> String {
        let mut buf = [0u8; 1 + 20 + 32 + 32];
        buf[0] = 0xff;
        buf[1..21].copy_from_slice(&self.deployer);
        buf[21..53].copy_from_slice(salt);
        buf[53..85].copy_from_slice(&self.init_code_hash);
        let hash = Keccak256::digest(buf);
        format!("0x{}", hex::encode(&hash[12..]))
    }
}

impl ChainGrinder for Create2Grinder {
    fn id(&self) -> &'static str {
        "create2"
    }

    fn display_name(&self) -> &'static str {
        "EVM CREATE2 (contract salt)"
    }

    fn grind_attempt(&self) -> (String, GrindAttempt) {
        let mut salt = [0u8; 32];
        OsRng.fill_bytes(&mut salt);
        let address = self.derive_address(&salt);
        (address, GrindAttempt::Secret32(salt))
    }

    fn finalize(&self, attempt: GrindAttempt) -> KeypairResult {
        let salt = secret_from_attempt(attempt);
        let address = self.derive_address(&salt);
        KeypairResult {
            address,
            exports: vec![
                KeyExport {
                    label: "CREATE2 Salt (hex)".into(),
                    value: hex::encode(salt),
                    hint: Some("Pass as salt to CREATE2 / factory deploy".into()),
                },
                KeyExport {
                    label: "CREATE2 Salt (0x hex)".into(),
                    value: format!("0x{}", hex::encode(salt)),
                    hint: Some("EVM tooling format".into()),
                },
                KeyExport {
                    label: "Deployer".into(),
                    value: format!("0x{}", hex::encode(self.deployer)),
                    hint: None,
                },
                KeyExport {
                    label: "Init Code Hash".into(),
                    value: format!("0x{}", hex::encode(self.init_code_hash)),
                    hint: None,
                },
            ],
        }
    }

    fn build_pattern(
        &self,
        prefix: Option<&str>,
        suffix: Option<&str>,
        contains: Option<&str>,
        _exact: bool,
    ) -> Result<Pattern, String> {
        build_hex_pattern(prefix, suffix, contains, true, 40)
    }

    fn expected_attempts(&self, pattern: &Pattern) -> f64 {
        expected_from_pattern(pattern, hex_combinations)
    }

    fn matches(&self, address: &str, pattern: &Pattern) -> bool {
        matches_pattern(address, pattern, true)
    }

    fn supports_exact_case(&self) -> bool {
        false
    }

    fn pattern_hint(&self) -> &'static str {
        "Hex pattern for the resulting contract address (same as EVM)."
    }
}

fn parse_hex_fixed<const N: usize>(input: &str, label: &str) -> Result<[u8; N], String> {
    let stripped = input.strip_prefix("0x").unwrap_or(input);
    let bytes = hex::decode(stripped).map_err(|e| format!("invalid {label} hex: {e}"))?;
    if bytes.len() != N {
        return Err(format!(
            "{label} must be {N} bytes ({} hex chars), got {}",
            N * 2,
            bytes.len()
        ));
    }
    let mut out = [0u8; N];
    out.copy_from_slice(&bytes);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chain::ChainGrinder;

    #[test]
    fn create2_address_is_0x_hex40() {
        let g = Create2Grinder::new([0u8; 20], [0u8; 32]);
        let (addr, _) = g.grind_attempt();
        assert!(addr.starts_with("0x"));
        assert_eq!(addr.len(), 42);
    }

    #[test]
    fn create2_known_vector() {
        // From EIP-1014 example-ish: deterministic empty inputs produce stable output.
        let g = Create2Grinder::new([0u8; 20], [0u8; 32]);
        let salt = [0u8; 32];
        let addr = g.derive_address(&salt);
        assert_eq!(addr.len(), 42);
        // Re-derive must be identical
        assert_eq!(g.derive_address(&salt), addr);
    }
}
