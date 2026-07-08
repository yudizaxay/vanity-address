mod evm;
mod solana;

pub use evm::EvmGrinder;
pub use solana::SolanaGrinder;

use crate::chain::{ChainGrinder, GrindAttempt, KeypairResult};
use crate::pattern::Pattern;

#[derive(Clone)]
pub enum Chain {
    Solana(SolanaGrinder),
    Evm(EvmGrinder),
}

impl Chain {
    pub fn from_id(id: &str) -> Result<Self, String> {
        match id.to_ascii_lowercase().as_str() {
            "sol" | "solana" => Ok(Chain::Solana(SolanaGrinder)),
            "evm" | "eth" | "ethereum" => Ok(Chain::Evm(EvmGrinder)),
            _ => Err(format!("Unknown chain '{id}'. Supported: sol, evm")),
        }
    }

    pub fn all_ids() -> &'static [&'static str] {
        &["sol", "evm"]
    }
}

impl ChainGrinder for Chain {
    fn id(&self) -> &'static str {
        match self {
            Chain::Solana(g) => g.id(),
            Chain::Evm(g) => g.id(),
        }
    }

    fn display_name(&self) -> &'static str {
        match self {
            Chain::Solana(g) => g.display_name(),
            Chain::Evm(g) => g.display_name(),
        }
    }

    fn grind_attempt(&self) -> (String, GrindAttempt) {
        match self {
            Chain::Solana(g) => g.grind_attempt(),
            Chain::Evm(g) => g.grind_attempt(),
        }
    }

    fn finalize(&self, attempt: GrindAttempt) -> KeypairResult {
        match self {
            Chain::Solana(g) => g.finalize(attempt),
            Chain::Evm(g) => g.finalize(attempt),
        }
    }

    fn build_pattern(
        &self,
        prefix: Option<&str>,
        suffix: Option<&str>,
        exact: bool,
    ) -> Result<Pattern, String> {
        match self {
            Chain::Solana(g) => g.build_pattern(prefix, suffix, exact),
            Chain::Evm(g) => g.build_pattern(prefix, suffix, exact),
        }
    }

    fn expected_attempts(&self, pattern: &Pattern) -> f64 {
        match self {
            Chain::Solana(g) => g.expected_attempts(pattern),
            Chain::Evm(g) => g.expected_attempts(pattern),
        }
    }

    fn matches(&self, address: &str, pattern: &Pattern) -> bool {
        match self {
            Chain::Solana(g) => g.matches(address, pattern),
            Chain::Evm(g) => g.matches(address, pattern),
        }
    }

    fn supports_exact_case(&self) -> bool {
        match self {
            Chain::Solana(g) => g.supports_exact_case(),
            Chain::Evm(g) => g.supports_exact_case(),
        }
    }

    fn pattern_hint(&self) -> &'static str {
        match self {
            Chain::Solana(g) => g.pattern_hint(),
            Chain::Evm(g) => g.pattern_hint(),
        }
    }
}
