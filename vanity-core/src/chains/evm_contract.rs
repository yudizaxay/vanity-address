//! EVM CREATE vanity — grind a deployer key so the contract it deploys with
//! its first transaction lands on a matching address:
//! `keccak256(rlp([deployer, 0]))[12:]`.

use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use rand::rngs::OsRng;
use secp256k1::{Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};

use super::util::{
    build_hex_pattern, expected_from_pattern, hex_combinations, matches_pattern,
    secret_from_attempt,
};

#[derive(Clone, Default)]
pub struct EvmContractGrinder;

impl EvmContractGrinder {
    fn deployer_bytes(secret_key: &SecretKey) -> [u8; 20] {
        let secp = Secp256k1::new();
        let public_key = secret_key.public_key(&secp);
        let hash = Keccak256::digest(&public_key.serialize_uncompressed()[1..]);
        let mut out = [0u8; 20];
        out.copy_from_slice(&hash[12..]);
        out
    }

    /// RLP of `[address, 0]` is fixed-size: list header, 20-byte string, empty-int.
    fn contract_address(deployer: &[u8; 20]) -> String {
        let mut buf = [0u8; 23];
        buf[0] = 0xd6;
        buf[1] = 0x94;
        buf[2..22].copy_from_slice(deployer);
        buf[22] = 0x80;
        let hash = Keccak256::digest(buf);
        format!("0x{}", hex::encode(&hash[12..]))
    }
}

impl ChainGrinder for EvmContractGrinder {
    fn id(&self) -> &'static str {
        "evm-contract"
    }

    fn display_name(&self) -> &'static str {
        "EVM contract (CREATE, nonce 0)"
    }

    fn grind_attempt(&self) -> (String, GrindAttempt) {
        let secret_key = SecretKey::new(&mut OsRng);
        let address = Self::contract_address(&Self::deployer_bytes(&secret_key));
        (address, GrindAttempt::Secret32(secret_key.secret_bytes()))
    }

    fn finalize(&self, attempt: GrindAttempt) -> KeypairResult {
        let secret_bytes = secret_from_attempt(attempt);
        let secret_key = SecretKey::from_slice(&secret_bytes).expect("valid secp256k1 secret");
        let deployer = Self::deployer_bytes(&secret_key);

        KeypairResult {
            address: Self::contract_address(&deployer),
            exports: vec![
                KeyExport {
                    label: "Deployer Private Key (hex)".into(),
                    value: hex::encode(secret_bytes),
                    hint: Some(
                        "Deploy from this account as its FIRST transaction (nonce 0) — fund it, don't use it for anything else first"
                            .into(),
                    ),
                },
                KeyExport {
                    label: "Deployer Private Key (0x hex)".into(),
                    value: format!("0x{}", hex::encode(secret_bytes)),
                    hint: Some("Foundry / Hardhat / ethers.js / viem".into()),
                },
                KeyExport {
                    label: "Deployer Address".into(),
                    value: format!("0x{}", hex::encode(deployer)),
                    hint: Some("Send gas here before deploying".into()),
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
        "Hex pattern for the contract address the deployer's first transaction creates."
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chain::ChainGrinder;

    #[test]
    fn create_nonce0_known_vector() {
        let deployer: [u8; 20] = hex::decode("6ac7ea33f8831ea9dcc53393aaa88b25a785dbf0")
            .unwrap()
            .try_into()
            .unwrap();
        assert_eq!(
            EvmContractGrinder::contract_address(&deployer),
            "0xcd234a471b72ba2f1ccf0a70fcaba648a5eecd8d"
        );
    }

    #[test]
    fn finalize_matches_grind_and_exports_deployer() {
        let g = EvmContractGrinder;
        let (addr, attempt) = g.grind_attempt();
        let kp = g.finalize(attempt);
        assert_eq!(kp.address, addr);
        assert_eq!(kp.address.len(), 42);
        let deployer = &kp.exports[2].value;
        assert!(deployer.starts_with("0x") && deployer.len() == 42);
        assert_ne!(deployer, &kp.address);
    }
}
