mod algorand;
mod aptos;
mod bitcoin_like;
mod cardano;
mod cosmos;
mod evm;
mod filecoin;
mod hedera;
mod icp;
mod kaspa;
mod near;
mod polkadot;
mod ripple;
mod solana;
mod stellar;
mod sui;
mod tezos;
mod ton;
mod tron;
mod util;

pub use algorand::AlgorandGrinder;
pub use aptos::AptosGrinder;
pub use bitcoin_like::BitcoinLikeGrinder;
pub use cardano::CardanoGrinder;
pub use cosmos::CosmosGrinder;
pub use evm::EvmGrinder;
pub use filecoin::FilecoinGrinder;
pub use hedera::HederaGrinder;
pub use icp::IcpGrinder;
pub use kaspa::KaspaGrinder;
pub use near::NearGrinder;
pub use polkadot::PolkadotGrinder;
pub use ripple::RippleGrinder;
pub use solana::SolanaGrinder;
pub use stellar::StellarGrinder;
pub use sui::SuiGrinder;
pub use tezos::TezosGrinder;
pub use ton::TonGrinder;
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
    Algorand(AlgorandGrinder),
    Tezos(TezosGrinder),
    Icp(IcpGrinder),
    Kaspa(KaspaGrinder),
    Ton(TonGrinder),
    Filecoin(FilecoinGrinder),
    Polkadot(PolkadotGrinder),
    Cardano(CardanoGrinder),
    Hedera(HederaGrinder),
}

/// Menu label for interactive chain picker (index 0-based).
/// Ordered A–Z by display name for easier selection.
pub const MENU_CHAINS: [(&str, &str); 22] = [
    ("algo", "Algorand (base32)"),
    ("aptos", "Aptos (0x hex)"),
    ("btc", "Bitcoin (base58 · P2PKH)"),
    ("ada", "Cardano (enterprise addr1)"),
    ("cosmos", "Cosmos (bech32 · ATOM)"),
    ("doge", "Dogecoin (base58)"),
    ("evm", "EVM (0x hex · MetaMask)"),
    ("fil", "Filecoin (f1 · secp256k1)"),
    ("hedera", "Hedera (ed25519 pubkey hex)"),
    ("icp", "Internet Computer (principal)"),
    ("kaspa", "Kaspa (bech32)"),
    ("ltc", "Litecoin (base58 · P2PKH)"),
    ("near", "NEAR (hex implicit account)"),
    ("osmo", "Osmosis (bech32 · OSMO)"),
    ("dot", "Polkadot (SS58 · ed25519)"),
    ("xrp", "Ripple (base58 · r…)"),
    ("sol", "Solana (base58 · Phantom, Solflare)"),
    ("xlm", "Stellar (strkey · G…)"),
    ("sui", "Sui (0x hex)"),
    ("xtz", "Tezos (tz1 · ed25519)"),
    ("ton", "TON (Wallet V4R2 · UQ…)"),
    ("trx", "Tron (base58 · T…)"),
];

impl Chain {
    pub fn from_menu_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Chain::Algorand(AlgorandGrinder)),
            1 => Some(Chain::Aptos(AptosGrinder)),
            2 => Some(Chain::Bitcoin(BitcoinLikeGrinder::bitcoin())),
            3 => Some(Chain::Cardano(CardanoGrinder)),
            4 => Some(Chain::Cosmos(CosmosGrinder::cosmos())),
            5 => Some(Chain::Dogecoin(BitcoinLikeGrinder::dogecoin())),
            6 => Some(Chain::Evm(EvmGrinder)),
            7 => Some(Chain::Filecoin(FilecoinGrinder)),
            8 => Some(Chain::Hedera(HederaGrinder)),
            9 => Some(Chain::Icp(IcpGrinder)),
            10 => Some(Chain::Kaspa(KaspaGrinder)),
            11 => Some(Chain::Litecoin(BitcoinLikeGrinder::litecoin())),
            12 => Some(Chain::Near(NearGrinder)),
            13 => Some(Chain::Osmosis(CosmosGrinder::osmosis())),
            14 => Some(Chain::Polkadot(PolkadotGrinder)),
            15 => Some(Chain::Ripple(RippleGrinder)),
            16 => Some(Chain::Solana(SolanaGrinder)),
            17 => Some(Chain::Stellar(StellarGrinder)),
            18 => Some(Chain::Sui(SuiGrinder)),
            19 => Some(Chain::Tezos(TezosGrinder)),
            20 => Some(Chain::Ton(TonGrinder)),
            21 => Some(Chain::Tron(TronGrinder)),
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
            "algo" | "algorand" => Ok(Chain::Algorand(AlgorandGrinder)),
            "xtz" | "tezos" => Ok(Chain::Tezos(TezosGrinder)),
            "icp" | "internet-computer" | "dfinity" => Ok(Chain::Icp(IcpGrinder)),
            "kaspa" | "kas" => Ok(Chain::Kaspa(KaspaGrinder)),
            "ton" => Ok(Chain::Ton(TonGrinder)),
            "fil" | "filecoin" => Ok(Chain::Filecoin(FilecoinGrinder)),
            "dot" | "polkadot" | "substrate" => Ok(Chain::Polkadot(PolkadotGrinder)),
            "ada" | "cardano" => Ok(Chain::Cardano(CardanoGrinder)),
            "hedera" | "hbar" => Ok(Chain::Hedera(HederaGrinder)),
            _ => Err(format!(
                "Unknown chain '{id}'. Supported: {}",
                Self::supported_ids_display()
            )),
        }
    }

    /// Chain IDs in the same A–Z menu order.
    pub fn all_ids() -> &'static [&'static str] {
        &[
            "algo", "aptos", "btc", "ada", "cosmos", "doge", "evm", "fil", "hedera", "icp",
            "kaspa", "ltc", "near", "osmo", "dot", "xrp", "sol", "xlm", "sui", "xtz", "ton", "trx",
        ]
    }

    fn supported_ids_display() -> String {
        Self::all_ids().join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::Chain;
    use crate::chain::ChainGrinder;

    #[test]
    fn all_menu_chains_resolve_and_grind() {
        assert_eq!(super::MENU_CHAINS.len(), 22);
        assert_eq!(Chain::all_ids().len(), 22);
        for (i, (id, _)) in super::MENU_CHAINS.iter().enumerate() {
            let chain = Chain::from_menu_index(i).expect("menu index");
            assert_eq!(chain.id(), *id);
            let via_id = Chain::from_id(id).expect("from_id");
            assert_eq!(via_id.id(), *id);
            let (addr, attempt) = chain.grind_attempt();
            assert!(!addr.is_empty(), "{id} empty address");
            let finalized = chain.finalize(attempt);
            assert_eq!(finalized.address, addr, "{id} finalize mismatch");
            assert!(!finalized.exports.is_empty(), "{id} missing exports");
        }
    }

    #[test]
    fn menu_chains_sorted_a_to_z_by_label() {
        let labels: Vec<&str> = super::MENU_CHAINS.iter().map(|(_, label)| *label).collect();
        let mut sorted = labels.clone();
        sorted.sort_by(|a, b| a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase()));
        assert_eq!(labels, sorted, "MENU_CHAINS must stay A–Z by display name");
        for (i, (id, _)) in super::MENU_CHAINS.iter().enumerate() {
            assert_eq!(Chain::all_ids()[i], *id);
        }
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
            Chain::Algorand(g) => g.$method($($arg),*),
            Chain::Tezos(g) => g.$method($($arg),*),
            Chain::Icp(g) => g.$method($($arg),*),
            Chain::Kaspa(g) => g.$method($($arg),*),
            Chain::Ton(g) => g.$method($($arg),*),
            Chain::Filecoin(g) => g.$method($($arg),*),
            Chain::Polkadot(g) => g.$method($($arg),*),
            Chain::Cardano(g) => g.$method($($arg),*),
            Chain::Hedera(g) => g.$method($($arg),*),
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
