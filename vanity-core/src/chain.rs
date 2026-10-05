use crate::pattern::Pattern;
use zeroize::Zeroize;

#[derive(Debug, Clone)]
pub struct KeyExport {
    pub label: String,
    pub value: String,
    pub hint: Option<String>,
}

impl Drop for KeyExport {
    fn drop(&mut self) {
        self.value.zeroize();
    }
}

#[derive(Debug, Clone)]
pub struct KeypairResult {
    pub address: String,
    pub exports: Vec<KeyExport>,
}

/// Lightweight per-attempt state — export formats are built only on a match.
pub enum GrindAttempt {
    /// secp256k1 / ed25519 secret key bytes (chain-specific finalize)
    Secret32([u8; 32]),
}

impl GrindAttempt {
    pub fn secret_bytes(&self) -> [u8; 32] {
        let GrindAttempt::Secret32(bytes) = self;
        *bytes
    }
}

impl Drop for GrindAttempt {
    fn drop(&mut self) {
        let GrindAttempt::Secret32(bytes) = self;
        bytes.zeroize();
    }
}

pub trait ChainGrinder: Send + Sync + Clone {
    fn id(&self) -> &'static str;
    fn display_name(&self) -> &'static str;

    /// Generate address + retain secret material for finalize on match.
    fn grind_attempt(&self) -> (String, GrindAttempt);

    /// Build export formats from a winning attempt (called once on success).
    fn finalize(&self, attempt: GrindAttempt) -> KeypairResult;

    fn build_pattern(
        &self,
        prefix: Option<&str>,
        suffix: Option<&str>,
        contains: Option<&str>,
        exact: bool,
    ) -> Result<Pattern, String>;
    fn expected_attempts(&self, pattern: &Pattern) -> f64;
    fn matches(&self, address: &str, pattern: &Pattern) -> bool;
    fn supports_exact_case(&self) -> bool;
    fn pattern_hint(&self) -> &'static str;
}
