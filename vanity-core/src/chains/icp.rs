use crate::chain::{ChainGrinder, GrindAttempt, KeyExport, KeypairResult};
use crate::pattern::Pattern;
use data_encoding::BASE32_NOPAD;
use sha2::{Digest, Sha224};
use solana_sdk::signature::{Keypair, Signer};

use super::util::{
    base32_combinations, build_base58_pattern, expected_from_pattern, grind_ed25519,
    keypair_from_secret, secret_from_attempt, BASE32_ALPHABET_LOWER,
};

#[derive(Clone, Default)]
pub struct IcpGrinder;

impl IcpGrinder {
    /// Self-authenticating principal from raw ed25519 public key.
    fn derive_address(keypair: &Keypair) -> String {
        let pubkey = keypair.pubkey().to_bytes();
        // SubjectPublicKeyInfo DER for Ed25519
        let mut der = vec![
            0x30, 0x2a, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x03, 0x21, 0x00,
        ];
        der.extend_from_slice(&pubkey);

        let hash = Sha224::digest(&der);
        let mut principal = Vec::with_capacity(29);
        principal.extend_from_slice(&hash);
        principal.push(0x02); // self-authenticating

        let checksum = crc32fast::hash(&principal);
        let mut blob = Vec::with_capacity(33);
        blob.extend_from_slice(&checksum.to_be_bytes());
        blob.extend_from_slice(&principal);

        let encoded = BASE32_NOPAD.encode(&blob).to_ascii_lowercase();
        let mut groups = Vec::new();
        for chunk in encoded.as_bytes().chunks(5) {
            groups.push(std::str::from_utf8(chunk).unwrap());
        }
        groups.join("-")
    }
}

impl ChainGrinder for IcpGrinder {
    fn id(&self) -> &'static str {
        "icp"
    }

    fn display_name(&self) -> &'static str {
        "Internet Computer"
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
                hint: Some("ICP identity / dfx principal".into()),
            }],
        }
    }

    fn build_pattern(
        &self,
        prefix: Option<&str>,
        suffix: Option<&str>,
        _exact: bool,
    ) -> Result<Pattern, String> {
        let normalize = |s: Option<&str>| {
            s.map(|v| {
                v.chars()
                    .filter(|c| *c != '-')
                    .collect::<String>()
                    .to_ascii_lowercase()
            })
        };
        let prefix_n = normalize(prefix);
        let suffix_n = normalize(suffix);
        build_base58_pattern(
            prefix_n.as_deref(),
            suffix_n.as_deref(),
            false,
            BASE32_ALPHABET_LOWER,
            63,
        )
    }

    fn expected_attempts(&self, pattern: &Pattern) -> f64 {
        expected_from_pattern(pattern, |p| {
            let stripped: String = p.chars().filter(|c| *c != '-').collect();
            base32_combinations(&stripped)
        })
    }

    fn matches(&self, address: &str, pattern: &Pattern) -> bool {
        let addr: String = address
            .chars()
            .filter(|c| *c != '-')
            .collect::<String>()
            .to_ascii_lowercase();
        let prefix: String = pattern.prefix_match.chars().filter(|c| *c != '-').collect();
        let suffix: String = pattern.suffix_match.chars().filter(|c| *c != '-').collect();
        crate::pattern::matches_both(&addr, &prefix, &suffix, true)
    }

    fn supports_exact_case(&self) -> bool {
        false
    }

    fn pattern_hint(&self) -> &'static str {
        "Base32 principal (dashes optional). Prefer --prefix; text always ends with 'e' (self-auth tag)."
    }
}

#[cfg(test)]
mod tests {
    use super::IcpGrinder;
    use crate::chain::ChainGrinder;

    #[test]
    fn icp_principal_has_dashes() {
        let g = IcpGrinder;
        let (addr, _) = g.grind_attempt();
        assert!(addr.contains('-'));
        assert!(addr.chars().all(|c| c.is_ascii_lowercase()
            || c.is_ascii_digit()
            || c == '-'
            || ('2'..='7').contains(&c)));
    }
}
