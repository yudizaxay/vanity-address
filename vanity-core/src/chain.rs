use crate::pattern::Pattern;

#[derive(Debug, Clone)]
pub struct KeyExport {
    pub label: String,
    pub value: String,
    pub hint: Option<String>,
}

#[derive(Debug, Clone)]
pub struct KeypairResult {
    pub address: String,
    pub exports: Vec<KeyExport>,
}

pub trait ChainGrinder: Send + Sync + Clone {
    fn id(&self) -> &'static str;
    fn display_name(&self) -> &'static str;
    fn generate_keypair(&self) -> KeypairResult;
    fn build_pattern(
        &self,
        prefix: Option<&str>,
        suffix: Option<&str>,
        exact: bool,
    ) -> Result<Pattern, String>;
    fn expected_attempts(&self, pattern: &Pattern) -> f64;
    fn matches(&self, address: &str, pattern: &Pattern) -> bool;
    fn supports_exact_case(&self) -> bool;
    fn pattern_hint(&self) -> &'static str;
}
