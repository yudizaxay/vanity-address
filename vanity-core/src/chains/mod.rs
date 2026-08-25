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
mod kusama;
mod multiversx;
mod near;
mod polkadot;
mod ripple;
#[cfg(not(target_arch = "wasm32"))]
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
pub use kusama::KusamaGrinder;
pub use multiversx::MultiversXGrinder;
pub use near::NearGrinder;
pub use polkadot::PolkadotGrinder;
pub use ripple::RippleGrinder;
#[cfg(not(target_arch = "wasm32"))]
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
    #[cfg(not(target_arch = "wasm32"))]
    Solana(SolanaGrinder),
    Evm(EvmGrinder),
    Bitcoin(BitcoinLikeGrinder),
    Litecoin(BitcoinLikeGrinder),
    Dogecoin(BitcoinLikeGrinder),
    Dash(BitcoinLikeGrinder),
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
    Kusama(KusamaGrinder),
    Ton(TonGrinder),
    Filecoin(FilecoinGrinder),
    Polkadot(PolkadotGrinder),
    Cardano(CardanoGrinder),
    Hedera(HederaGrinder),
    MultiversX(MultiversXGrinder),
}

/// Menu label for interactive chain picker (index 0-based).
/// Ordered A–Z by display name for easier selection.
#[cfg(not(target_arch = "wasm32"))]
pub const MENU_CHAINS: [(&str, &str); 25] = [
    ("algo", "Algorand (base32)"),
    ("aptos", "Aptos (0x hex)"),
    ("btc", "Bitcoin (base58 · P2PKH)"),
    ("ada", "Cardano (enterprise addr1)"),
    ("cosmos", "Cosmos (bech32 · ATOM)"),
    ("dash", "Dash (base58 · P2PKH)"),
    ("doge", "Dogecoin (base58)"),
    ("evm", "EVM (0x hex · MetaMask)"),
    ("fil", "Filecoin (f1 · secp256k1)"),
    ("hedera", "Hedera (ed25519 pubkey hex)"),
    ("icp", "Internet Computer (principal)"),
    ("kaspa", "Kaspa (bech32)"),
    ("ksm", "Kusama (SS58 · ed25519)"),
    ("ltc", "Litecoin (base58 · P2PKH)"),
    ("erd", "MultiversX (bech32 · erd1)"),
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

#[cfg(target_arch = "wasm32")]
pub const MENU_CHAINS: [(&str, &str); 24] = [
    ("algo", "Algorand (base32)"),
    ("aptos", "Aptos (0x hex)"),
    ("btc", "Bitcoin (base58 · P2PKH)"),
    ("ada", "Cardano (enterprise addr1)"),
    ("cosmos", "Cosmos (bech32 · ATOM)"),
    ("dash", "Dash (base58 · P2PKH)"),
    ("doge", "Dogecoin (base58)"),
    ("evm", "EVM (0x hex · MetaMask)"),
    ("fil", "Filecoin (f1 · secp256k1)"),
    ("hedera", "Hedera (ed25519 pubkey hex)"),
    ("icp", "Internet Computer (principal)"),
    ("kaspa", "Kaspa (bech32)"),
    ("ksm", "Kusama (SS58 · ed25519)"),
    ("ltc", "Litecoin (base58 · P2PKH)"),
    ("erd", "MultiversX (bech32 · erd1)"),
    ("near", "NEAR (hex implicit account)"),
    ("osmo", "Osmosis (bech32 · OSMO)"),
    ("dot", "Polkadot (SS58 · ed25519)"),
    ("xrp", "Ripple (base58 · r…)"),
    ("xlm", "Stellar (strkey · G…)"),
    ("sui", "Sui (0x hex)"),
    ("xtz", "Tezos (tz1 · ed25519)"),
    ("ton", "TON (Wallet V4R2 · UQ…)"),
    ("trx", "Tron (base58 · T…)"),
];

impl Chain {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_menu_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Chain::Algorand(AlgorandGrinder)),
            1 => Some(Chain::Aptos(AptosGrinder)),
            2 => Some(Chain::Bitcoin(BitcoinLikeGrinder::bitcoin())),
            3 => Some(Chain::Cardano(CardanoGrinder)),
            4 => Some(Chain::Cosmos(CosmosGrinder::cosmos())),
            5 => Some(Chain::Dash(BitcoinLikeGrinder::dash())),
            6 => Some(Chain::Dogecoin(BitcoinLikeGrinder::dogecoin())),
            7 => Some(Chain::Evm(EvmGrinder)),
            8 => Some(Chain::Filecoin(FilecoinGrinder)),
            9 => Some(Chain::Hedera(HederaGrinder)),
            10 => Some(Chain::Icp(IcpGrinder)),
            11 => Some(Chain::Kaspa(KaspaGrinder)),
            12 => Some(Chain::Kusama(KusamaGrinder)),
            13 => Some(Chain::Litecoin(BitcoinLikeGrinder::litecoin())),
            14 => Some(Chain::MultiversX(MultiversXGrinder)),
            15 => Some(Chain::Near(NearGrinder)),
            16 => Some(Chain::Osmosis(CosmosGrinder::osmosis())),
            17 => Some(Chain::Polkadot(PolkadotGrinder)),
            18 => Some(Chain::Ripple(RippleGrinder)),
            19 => Some(Chain::Solana(SolanaGrinder)),
            20 => Some(Chain::Stellar(StellarGrinder)),
            21 => Some(Chain::Sui(SuiGrinder)),
            22 => Some(Chain::Tezos(TezosGrinder)),
            23 => Some(Chain::Ton(TonGrinder)),
            24 => Some(Chain::Tron(TronGrinder)),
            _ => None,
        }
    }

    /// wasm32 has no Solana chain, so `MENU_CHAINS` here is 24 entries with
    /// indices 19-23 shifted down by one (Stellar/Sui/Tezos/Ton/Tron) versus
    /// the native 25-entry array — this mirrors that shift exactly.
    #[cfg(target_arch = "wasm32")]
    pub fn from_menu_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Chain::Algorand(AlgorandGrinder)),
            1 => Some(Chain::Aptos(AptosGrinder)),
            2 => Some(Chain::Bitcoin(BitcoinLikeGrinder::bitcoin())),
            3 => Some(Chain::Cardano(CardanoGrinder)),
            4 => Some(Chain::Cosmos(CosmosGrinder::cosmos())),
            5 => Some(Chain::Dash(BitcoinLikeGrinder::dash())),
            6 => Some(Chain::Dogecoin(BitcoinLikeGrinder::dogecoin())),
            7 => Some(Chain::Evm(EvmGrinder)),
            8 => Some(Chain::Filecoin(FilecoinGrinder)),
            9 => Some(Chain::Hedera(HederaGrinder)),
            10 => Some(Chain::Icp(IcpGrinder)),
            11 => Some(Chain::Kaspa(KaspaGrinder)),
            12 => Some(Chain::Kusama(KusamaGrinder)),
            13 => Some(Chain::Litecoin(BitcoinLikeGrinder::litecoin())),
            14 => Some(Chain::MultiversX(MultiversXGrinder)),
            15 => Some(Chain::Near(NearGrinder)),
            16 => Some(Chain::Osmosis(CosmosGrinder::osmosis())),
            17 => Some(Chain::Polkadot(PolkadotGrinder)),
            18 => Some(Chain::Ripple(RippleGrinder)),
            19 => Some(Chain::Stellar(StellarGrinder)),
            20 => Some(Chain::Sui(SuiGrinder)),
            21 => Some(Chain::Tezos(TezosGrinder)),
            22 => Some(Chain::Ton(TonGrinder)),
            23 => Some(Chain::Tron(TronGrinder)),
            _ => None,
        }
    }

    pub fn from_id(id: &str) -> Result<Self, String> {
        let id = id.to_ascii_lowercase();
        match id.as_str() {
            #[cfg(not(target_arch = "wasm32"))]
            "sol" | "solana" => Ok(Chain::Solana(SolanaGrinder)),
            "evm" | "eth" | "ethereum" => Ok(Chain::Evm(EvmGrinder)),
            "btc" | "bitcoin" => Ok(Chain::Bitcoin(BitcoinLikeGrinder::bitcoin())),
            "ltc" | "litecoin" => Ok(Chain::Litecoin(BitcoinLikeGrinder::litecoin())),
            "doge" | "dogecoin" => Ok(Chain::Dogecoin(BitcoinLikeGrinder::dogecoin())),
            "dash" => Ok(Chain::Dash(BitcoinLikeGrinder::dash())),
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
            "ksm" | "kusama" => Ok(Chain::Kusama(KusamaGrinder)),
            "ton" => Ok(Chain::Ton(TonGrinder)),
            "fil" | "filecoin" => Ok(Chain::Filecoin(FilecoinGrinder)),
            "dot" | "polkadot" | "substrate" => Ok(Chain::Polkadot(PolkadotGrinder)),
            "ada" | "cardano" => Ok(Chain::Cardano(CardanoGrinder)),
            "hedera" | "hbar" => Ok(Chain::Hedera(HederaGrinder)),
            "erd" | "mvx" | "elrond" | "multiversx" => Ok(Chain::MultiversX(MultiversXGrinder)),
            _ => Err(format!(
                "Unknown chain '{id}'. Supported: {}",
                Self::supported_ids_display()
            )),
        }
    }

    /// Chain IDs in the same A–Z menu order.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn all_ids() -> &'static [&'static str] {
        &[
            "algo", "aptos", "btc", "ada", "cosmos", "dash", "doge", "evm", "fil", "hedera", "icp",
            "kaspa", "ksm", "ltc", "erd", "near", "osmo", "dot", "xrp", "sol", "xlm", "sui", "xtz",
            "ton", "trx",
        ]
    }

    /// Chain IDs in the same A–Z menu order.
    #[cfg(target_arch = "wasm32")]
    pub fn all_ids() -> &'static [&'static str] {
        &[
            "algo", "aptos", "btc", "ada", "cosmos", "dash", "doge", "evm", "fil", "hedera", "icp",
            "kaspa", "ksm", "ltc", "erd", "near", "osmo", "dot", "xrp", "xlm", "sui", "xtz", "ton",
            "trx",
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
        assert_eq!(super::MENU_CHAINS.len(), 25);
        assert_eq!(Chain::all_ids().len(), 25);
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

    /// Documents/asserts the invariant that `from_menu_index` and
    /// `MENU_CHAINS` must stay in lockstep: for every index i,
    /// `from_menu_index(i).id() == MENU_CHAINS[i].0`. This is the exact
    /// invariant that regressed on wasm32 (index misalignment after Solana
    /// was excluded) — only runs on native today since this project has no
    /// wasm32 test harness yet, but it covers the native 25-entry array,
    /// and the wasm32 `from_menu_index`/`MENU_CHAINS` were hand-verified to
    /// carry the same shifted mapping.
    #[test]
    fn from_menu_index_matches_menu_chains_order() {
        for (i, (id, _)) in super::MENU_CHAINS.iter().enumerate() {
            let chain = Chain::from_menu_index(i).expect("menu index");
            assert_eq!(
                chain.id(),
                *id,
                "from_menu_index({i}) misaligned with MENU_CHAINS"
            );
        }
        assert!(Chain::from_menu_index(super::MENU_CHAINS.len()).is_none());
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
            #[cfg(not(target_arch = "wasm32"))]
            Chain::Solana(g) => g.$method($($arg),*),
            Chain::Evm(g) => g.$method($($arg),*),
            Chain::Bitcoin(g) => g.$method($($arg),*),
            Chain::Litecoin(g) => g.$method($($arg),*),
            Chain::Dogecoin(g) => g.$method($($arg),*),
            Chain::Dash(g) => g.$method($($arg),*),
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
            Chain::Kusama(g) => g.$method($($arg),*),
            Chain::Ton(g) => g.$method($($arg),*),
            Chain::Filecoin(g) => g.$method($($arg),*),
            Chain::Polkadot(g) => g.$method($($arg),*),
            Chain::Cardano(g) => g.$method($($arg),*),
            Chain::Hedera(g) => g.$method($($arg),*),
            Chain::MultiversX(g) => g.$method($($arg),*),
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
