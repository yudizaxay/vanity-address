mod aptos;
mod bitcoin_like;
mod cosmos;
mod evm;
mod near;
mod ripple;
mod solana;
mod stellar;
mod sui;
mod tron;
mod util;

pub use aptos::AptosGrinder;
pub use bitcoin_like::BitcoinLikeGrinder;
pub use cosmos::CosmosGrinder;
pub use evm::EvmGrinder;
pub use near::NearGrinder;
pub use ripple::RippleGrinder;
pub use solana::SolanaGrinder;
pub use stellar::StellarGrinder;
pub use sui::SuiGrinder;
pub use tron::TronGrinder;

use crate::chain::{ChainGrinder, GrindAttempt, KeypairResult};
use crate::pattern::Pattern;

#[derive(Clone)]
pub enum Chain {
    Solana(SolanaGrinder),
    Evm(EvmGrinder),
    Bitcoin(BitcoinLikeGrinder),
    Litecoin(BitcoinLikeGrinder),
    Dogecoin(BitcoinLikeGrinder),
    Tron(TronGrinder),
    Cosmos(CosmosGrinder),
    Osmosis(CosmosGrinder),
    Ripple(RippleGrinder),
    Stellar(StellarGrinder),
    Aptos(AptosGrinder),
    Sui(SuiGrinder),
    Near(NearGrinder),
}

/// Menu label for interactive chain picker (index 0-based).
pub const MENU_CHAINS: [(&str, &str); 13] = [
    ("sol", "Solana (base58 · Phantom, Solflare)"),
    ("evm", "EVM (0x hex · MetaMask)"),
    ("btc", "Bitcoin (base58 · P2PKH)"),
    ("ltc", "Litecoin (base58 · P2PKH)"),
    ("doge", "Dogecoin (base58)"),
    ("trx", "Tron (base58 · T…)"),
    ("cosmos", "Cosmos (bech32 · ATOM)"),
    ("osmo", "Osmosis (bech32 · OSMO)"),
    ("xrp", "Ripple (base58 · r…)"),
    ("xlm", "Stellar (strkey · G…)"),
    ("aptos", "Aptos (0x hex)"),
    ("sui", "Sui (0x hex)"),
    ("near", "NEAR (hex implicit account)"),
];

impl Chain {
    pub fn from_menu_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Chain::Solana(SolanaGrinder)),
            1 => Some(Chain::Evm(EvmGrinder)),
            2 => Some(Chain::Bitcoin(BitcoinLikeGrinder::bitcoin())),
            3 => Some(Chain::Litecoin(BitcoinLikeGrinder::litecoin())),
            4 => Some(Chain::Dogecoin(BitcoinLikeGrinder::dogecoin())),
            5 => Some(Chain::Tron(TronGrinder)),
            6 => Some(Chain::Cosmos(CosmosGrinder::cosmos())),
            7 => Some(Chain::Osmosis(CosmosGrinder::osmosis())),
            8 => Some(Chain::Ripple(RippleGrinder)),
            9 => Some(Chain::Stellar(StellarGrinder)),
            10 => Some(Chain::Aptos(AptosGrinder)),
            11 => Some(Chain::Sui(SuiGrinder)),
            12 => Some(Chain::Near(NearGrinder)),
            _ => None,
        }
    }

    pub fn from_id(id: &str) -> Result<Self, String> {
        let id = id.to_ascii_lowercase();
        match id.as_str() {
            "sol" | "solana" => Ok(Chain::Solana(SolanaGrinder)),
            "evm" | "eth" | "ethereum" => Ok(Chain::Evm(EvmGrinder)),
            "btc" | "bitcoin" => Ok(Chain::Bitcoin(BitcoinLikeGrinder::bitcoin())),
            "ltc" | "litecoin" => Ok(Chain::Litecoin(BitcoinLikeGrinder::litecoin())),
            "doge" | "dogecoin" => Ok(Chain::Dogecoin(BitcoinLikeGrinder::dogecoin())),
            "trx" | "tron" => Ok(Chain::Tron(TronGrinder)),
            "cosmos" | "atom" => Ok(Chain::Cosmos(CosmosGrinder::cosmos())),
            "osmo" | "osmosis" => Ok(Chain::Osmosis(CosmosGrinder::osmosis())),
            "xrp" | "ripple" => Ok(Chain::Ripple(RippleGrinder)),
            "xlm" | "stellar" => Ok(Chain::Stellar(StellarGrinder)),
            "aptos" | "apt" => Ok(Chain::Aptos(AptosGrinder)),
            "sui" => Ok(Chain::Sui(SuiGrinder)),
            "near" => Ok(Chain::Near(NearGrinder)),
            _ => Err(format!(
                "Unknown chain '{id}'. Supported: {}",
                Self::supported_ids_display()
            )),
        }
    }

    pub fn all_ids() -> &'static [&'static str] {
        &[
            "sol", "evm", "btc", "ltc", "doge", "trx", "cosmos", "osmo", "xrp", "xlm", "aptos",
            "sui", "near",
        ]
    }

    fn supported_ids_display() -> String {
        Self::all_ids().join(", ")
    }
}

macro_rules! dispatch {
    ($self:expr, $method:ident ( $($arg:expr),* $(,)? )) => {
        match $self {
            Chain::Solana(g) => g.$method($($arg),*),
            Chain::Evm(g) => g.$method($($arg),*),
            Chain::Bitcoin(g) => g.$method($($arg),*),
            Chain::Litecoin(g) => g.$method($($arg),*),
            Chain::Dogecoin(g) => g.$method($($arg),*),
            Chain::Tron(g) => g.$method($($arg),*),
            Chain::Cosmos(g) => g.$method($($arg),*),
            Chain::Osmosis(g) => g.$method($($arg),*),
            Chain::Ripple(g) => g.$method($($arg),*),
            Chain::Stellar(g) => g.$method($($arg),*),
            Chain::Aptos(g) => g.$method($($arg),*),
            Chain::Sui(g) => g.$method($($arg),*),
            Chain::Near(g) => g.$method($($arg),*),
        }
    };
}

impl ChainGrinder for Chain {
    fn id(&self) -> &'static str {
        dispatch!(self, id())
    }

    fn display_name(&self) -> &'static str {
        dispatch!(self, display_name())
    }

    fn grind_attempt(&self) -> (String, GrindAttempt) {
        dispatch!(self, grind_attempt())
    }

    fn finalize(&self, attempt: GrindAttempt) -> KeypairResult {
        dispatch!(self, finalize(attempt))
    }

    fn build_pattern(
        &self,
        prefix: Option<&str>,
        suffix: Option<&str>,
        exact: bool,
    ) -> Result<Pattern, String> {
        dispatch!(self, build_pattern(prefix, suffix, exact))
    }

    fn expected_attempts(&self, pattern: &Pattern) -> f64 {
        dispatch!(self, expected_attempts(pattern))
    }

    fn matches(&self, address: &str, pattern: &Pattern) -> bool {
        dispatch!(self, matches(address, pattern))
    }

    fn supports_exact_case(&self) -> bool {
        dispatch!(self, supports_exact_case())
    }

    fn pattern_hint(&self) -> &'static str {
        dispatch!(self, pattern_hint())
    }
}
