mod algorand;
mod aptos;
mod base58_odds;
mod bitcoin_like;
mod bitcoin_segwit;
mod cardano;
mod cosmos;
pub mod create2;
mod evm;
mod evm_contract;
mod filecoin;
mod hedera;
mod icp;
mod kaspa;
mod kusama;
mod multiversx;
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
pub use bitcoin_segwit::BitcoinSegwitGrinder;
pub use cardano::CardanoGrinder;
pub use cosmos::CosmosGrinder;
pub use create2::Create2Grinder;
pub use evm::EvmGrinder;
pub use evm_contract::EvmContractGrinder;
pub use filecoin::FilecoinGrinder;
pub use hedera::HederaGrinder;
pub use icp::IcpGrinder;
pub use kaspa::KaspaGrinder;
pub use kusama::KusamaGrinder;
pub use multiversx::MultiversXGrinder;
pub use near::NearGrinder;
pub use polkadot::PolkadotGrinder;
pub use ripple::RippleGrinder;
pub use solana::{SolanaGrinder, SolanaMintGrinder};
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
    BitcoinSegwit(BitcoinSegwitGrinder),
    BitcoinTaproot(BitcoinSegwitGrinder),
    Litecoin(BitcoinLikeGrinder),
    Dogecoin(BitcoinLikeGrinder),
    Dash(BitcoinLikeGrinder),
    Tron(TronGrinder),
    Cosmos(CosmosGrinder),
    Osmosis(CosmosGrinder),
    Sei(CosmosGrinder),
    Injective(CosmosGrinder),
    Celestia(CosmosGrinder),
    Dydx(CosmosGrinder),
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
    Create2(Create2Grinder),
    EvmContract(EvmContractGrinder),
    SolanaMint(SolanaMintGrinder),
}

/// Menu label for interactive chain picker (index 0-based).
/// Ordered A–Z by display name for easier selection.
pub const MENU_CHAINS: [(&str, &str); 31] = [
    ("algo", "Algorand (base32)"),
    ("aptos", "Aptos (0x hex)"),
    ("btc", "Bitcoin (base58 · P2PKH)"),
    ("btc-segwit", "Bitcoin SegWit (bc1q · P2WPKH)"),
    ("btc-taproot", "Bitcoin Taproot (bc1p)"),
    ("ada", "Cardano (enterprise addr1)"),
    ("tia", "Celestia (bech32 · TIA)"),
    ("cosmos", "Cosmos (bech32 · ATOM)"),
    ("dash", "Dash (base58 · P2PKH)"),
    ("doge", "Dogecoin (base58)"),
    ("dydx", "dYdX (bech32)"),
    ("evm", "EVM (0x hex · MetaMask / Base / Robinhood…)"),
    ("fil", "Filecoin (f1 · secp256k1)"),
    ("hedera", "Hedera (ed25519 pubkey hex)"),
    ("inj", "Injective (bech32 · INJ)"),
    ("icp", "Internet Computer (principal)"),
    ("kaspa", "Kaspa (bech32)"),
    ("ksm", "Kusama (SS58 · ed25519)"),
    ("ltc", "Litecoin (base58 · P2PKH)"),
    ("erd", "MultiversX (bech32 · erd1)"),
    ("near", "NEAR (hex implicit account)"),
    ("osmo", "Osmosis (bech32 · OSMO)"),
    ("dot", "Polkadot (SS58 · ed25519)"),
    ("xrp", "Ripple (base58 · r…)"),
    ("sei", "Sei (bech32 · SEI)"),
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
            3 => Some(Chain::BitcoinSegwit(BitcoinSegwitGrinder::segwit())),
            4 => Some(Chain::BitcoinTaproot(BitcoinSegwitGrinder::taproot())),
            5 => Some(Chain::Cardano(CardanoGrinder)),
            6 => Some(Chain::Celestia(CosmosGrinder::celestia())),
            7 => Some(Chain::Cosmos(CosmosGrinder::cosmos())),
            8 => Some(Chain::Dash(BitcoinLikeGrinder::dash())),
            9 => Some(Chain::Dogecoin(BitcoinLikeGrinder::dogecoin())),
            10 => Some(Chain::Dydx(CosmosGrinder::dydx())),
            11 => Some(Chain::Evm(EvmGrinder)),
            12 => Some(Chain::Filecoin(FilecoinGrinder)),
            13 => Some(Chain::Hedera(HederaGrinder)),
            14 => Some(Chain::Injective(CosmosGrinder::injective())),
            15 => Some(Chain::Icp(IcpGrinder)),
            16 => Some(Chain::Kaspa(KaspaGrinder)),
            17 => Some(Chain::Kusama(KusamaGrinder)),
            18 => Some(Chain::Litecoin(BitcoinLikeGrinder::litecoin())),
            19 => Some(Chain::MultiversX(MultiversXGrinder)),
            20 => Some(Chain::Near(NearGrinder)),
            21 => Some(Chain::Osmosis(CosmosGrinder::osmosis())),
            22 => Some(Chain::Polkadot(PolkadotGrinder)),
            23 => Some(Chain::Ripple(RippleGrinder)),
            24 => Some(Chain::Sei(CosmosGrinder::sei())),
            25 => Some(Chain::Solana(SolanaGrinder)),
            26 => Some(Chain::Stellar(StellarGrinder)),
            27 => Some(Chain::Sui(SuiGrinder)),
            28 => Some(Chain::Tezos(TezosGrinder)),
            29 => Some(Chain::Ton(TonGrinder)),
            30 => Some(Chain::Tron(TronGrinder)),
            _ => None,
        }
    }

    pub fn from_id(id: &str) -> Result<Self, String> {
        let id = id.to_ascii_lowercase();
        match id.as_str() {
            "sol" | "solana" => Ok(Chain::Solana(SolanaGrinder)),
            "sol-mint" | "spl-mint" | "mint" => Ok(Chain::SolanaMint(SolanaMintGrinder)),
            // EVM + trending L2 / app-chain aliases (same address math)
            "evm" | "eth" | "ethereum" | "base" | "arb" | "arbitrum" | "op" | "optimism"
            | "polygon" | "matic" | "avax" | "avalanche" | "bnb" | "bsc" | "fantom" | "ftm"
            | "scroll" | "linea" | "zksync" | "blast" | "mantle" | "mode" | "bera"
            | "berachain" | "monad" | "abstract" | "unichain" | "robinhood" | "hood" | "rh"
            | "hyperliquid" | "hyperevm" | "hype" | "sonic" => Ok(Chain::Evm(EvmGrinder)),
            "evm-contract" | "evm-create" | "contract" => {
                Ok(Chain::EvmContract(EvmContractGrinder))
            }
            "btc" | "bitcoin" => Ok(Chain::Bitcoin(BitcoinLikeGrinder::bitcoin())),
            "btc-segwit" | "segwit" | "bc1q" => {
                Ok(Chain::BitcoinSegwit(BitcoinSegwitGrinder::segwit()))
            }
            "btc-taproot" | "taproot" | "bc1p" => {
                Ok(Chain::BitcoinTaproot(BitcoinSegwitGrinder::taproot()))
            }
            "ltc" | "litecoin" => Ok(Chain::Litecoin(BitcoinLikeGrinder::litecoin())),
            "doge" | "dogecoin" => Ok(Chain::Dogecoin(BitcoinLikeGrinder::dogecoin())),
            "dash" => Ok(Chain::Dash(BitcoinLikeGrinder::dash())),
            "trx" | "tron" => Ok(Chain::Tron(TronGrinder)),
            "cosmos" | "atom" => Ok(Chain::Cosmos(CosmosGrinder::cosmos())),
            "osmo" | "osmosis" => Ok(Chain::Osmosis(CosmosGrinder::osmosis())),
            "sei" => Ok(Chain::Sei(CosmosGrinder::sei())),
            "inj" | "injective" => Ok(Chain::Injective(CosmosGrinder::injective())),
            "tia" | "celestia" => Ok(Chain::Celestia(CosmosGrinder::celestia())),
            "dydx" => Ok(Chain::Dydx(CosmosGrinder::dydx())),
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
                "Unknown chain '{id}'. Supported: {} (deploy modes: evm-contract, sol-mint)",
                Self::supported_ids_display()
            )),
        }
    }

    /// Chain IDs in the same A–Z menu order.
    pub fn all_ids() -> &'static [&'static str] {
        &[
            "algo",
            "aptos",
            "btc",
            "btc-segwit",
            "btc-taproot",
            "ada",
            "tia",
            "cosmos",
            "dash",
            "doge",
            "dydx",
            "evm",
            "fil",
            "hedera",
            "inj",
            "icp",
            "kaspa",
            "ksm",
            "ltc",
            "erd",
            "near",
            "osmo",
            "dot",
            "xrp",
            "sei",
            "sol",
            "xlm",
            "sui",
            "xtz",
            "ton",
            "trx",
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
            Chain::BitcoinSegwit(g) => g.$method($($arg),*),
            Chain::BitcoinTaproot(g) => g.$method($($arg),*),
            Chain::Litecoin(g) => g.$method($($arg),*),
            Chain::Dogecoin(g) => g.$method($($arg),*),
            Chain::Dash(g) => g.$method($($arg),*),
            Chain::Tron(g) => g.$method($($arg),*),
            Chain::Cosmos(g) => g.$method($($arg),*),
            Chain::Osmosis(g) => g.$method($($arg),*),
            Chain::Sei(g) => g.$method($($arg),*),
            Chain::Injective(g) => g.$method($($arg),*),
            Chain::Celestia(g) => g.$method($($arg),*),
            Chain::Dydx(g) => g.$method($($arg),*),
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
            Chain::Create2(g) => g.$method($($arg),*),
            Chain::EvmContract(g) => g.$method($($arg),*),
            Chain::SolanaMint(g) => g.$method($($arg),*),
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
        contains: Option<&str>,
        exact: bool,
    ) -> Result<Pattern, String> {
        dispatch!(self, build_pattern(prefix, suffix, contains, exact))
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

#[cfg(test)]
mod tests {
    use super::Chain;
    use crate::chain::ChainGrinder;

    #[test]
    fn all_menu_chains_resolve_and_grind() {
        assert_eq!(super::MENU_CHAINS.len(), 31);
        assert_eq!(Chain::all_ids().len(), 31);
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
        sorted.sort_by_key(|a| a.to_ascii_lowercase());
        assert_eq!(labels, sorted, "MENU_CHAINS must stay A–Z by display name");
        for (i, (id, _)) in super::MENU_CHAINS.iter().enumerate() {
            assert_eq!(Chain::all_ids()[i], *id);
        }
    }

    #[test]
    fn evm_trending_aliases_resolve() {
        for id in [
            "robinhood",
            "hood",
            "base",
            "arb",
            "optimism",
            "polygon",
            "avax",
            "bnb",
            "monad",
            "hyperliquid",
            "hyperevm",
            "sonic",
        ] {
            let c = Chain::from_id(id).unwrap_or_else(|e| panic!("{id}: {e}"));
            assert_eq!(c.id(), "evm");
        }
    }

    #[test]
    fn chain_forced_prefix_is_not_counted_as_difficulty() {
        for (id, input) in [
            ("evm", "ab"),
            ("evm-contract", "ab"),
            ("kaspa", "pq"),
            ("cosmos", "qp"),
            ("sei", "qp"),
            ("btc-segwit", "qp"),
            ("btc-taproot", "qp"),
            ("ada", "8x"),
            ("erd", "qp"),
            ("ton", "ab"),
            ("xlm", "ab"),
            ("xtz", "ab"),
            ("fil", "ab"),
            ("btc", "ab"),
            ("ltc", "ab"),
            ("doge", "ab"),
            ("dash", "ph"),
            ("trx", "ab"),
            ("xrp", "ab"),
            ("dot", "ab"),
        ] {
            let chain = Chain::from_id(id).unwrap();
            let p = chain
                .build_pattern(Some(input), None, None, false)
                .unwrap_or_else(|e| panic!("{id} {input}: {e}"));
            assert_eq!(
                p.effective_literal_chars(),
                2,
                "{id} '{input}' → '{}'",
                p.prefix
            );
            let est = crate::grind_estimate(chain.expected_attempts(&p), 500_000.0, &p);
            assert_eq!(est.risk, crate::PatternRisk::None, "{id} '{input}'");
        }
    }

    #[test]
    fn impossible_address_starts_are_rejected() {
        for (id, input) in [
            ("kaspa", "ax"),
            ("ada", "ax"),
            ("ada", "va"),
            ("ton", "zz"),
            ("xlm", "XY"),
            ("xlm", "GZ"),
            ("doge", "z"),
            ("ltc", "1"),
            ("dash", "1"),
            ("trx", "1"),
            ("xtz", "1"),
            ("ksm", "a"),
        ] {
            let chain = Chain::from_id(id).unwrap();
            let err = chain
                .build_pattern(Some(input), None, None, false)
                .expect_err(&format!("{id} '{input}' should be rejected"));
            assert!(err.contains("can never"), "{id} '{input}': {err}");
        }
    }

    #[test]
    fn constrained_start_lowers_expected_attempts() {
        let chain = Chain::from_id("kaspa").unwrap();
        let p = chain.build_pattern(Some("qp"), None, None, false).unwrap();
        assert_eq!(p.prefix, "kaspa:qp");
        // `q` is certain, `p` is 1 of 4 → about 4 tries, not 32².
        assert!((chain.expected_attempts(&p) - 4.0).abs() < 1e-9);

        let btc = Chain::from_id("btc").unwrap();
        let p = btc.build_pattern(Some("1ab"), None, None, false).unwrap();
        assert_eq!(p.prefix, "1ab");
        assert_eq!(p.effective_literal_chars(), 2);

        // Doge is `D` + one of 24 characters, so `Da` takes ~24 tries, not 58.
        let doge = Chain::from_id("doge").unwrap();
        let p = doge.build_pattern(Some("A"), None, None, true).unwrap();
        let tries = doge.expected_attempts(&p);
        assert!((20.0..30.0).contains(&tries), "doge 'A' → {tries}");
    }

    #[test]
    fn ambiguous_lead_picks_the_reading_that_exists() {
        // `D1` can't follow the `D` lead, so it must mean `DD1…`.
        for (id, input, want) in [
            ("doge", "D1", "DD1"),
            ("trx", "T1", "TT1"),
            ("ltc", "L1", "LL1"),
            ("kaspa", "q0", "kaspa:qq0"),
            ("kaspa", "qp", "kaspa:qp"),
            ("ada", "8", "addr1v8"),
            ("btc", "1ab", "1ab"),
        ] {
            let chain = Chain::from_id(id).unwrap();
            let p = chain.build_pattern(Some(input), None, None, true).unwrap();
            assert_eq!(p.prefix, want, "{id} '{input}'");
        }
    }

    /// Every real address must be reachable: the start of a freshly generated
    /// address is never rejected and the resulting pattern matches it.
    #[test]
    fn real_address_starts_are_always_accepted() {
        let mut ids: Vec<&str> = Chain::all_ids().to_vec();
        ids.extend(["evm-contract", "sol-mint"]);
        for id in ids {
            let chain = Chain::from_id(id).unwrap();
            let addrs: Vec<String> = (0..300).map(|_| chain.grind_attempt().0).collect();
            let fixed = (0..addrs[0].len())
                .take_while(|&i| {
                    addrs
                        .iter()
                        .all(|a| a.as_bytes()[i] == addrs[0].as_bytes()[i])
                })
                .count();
            for a in &addrs {
                for k in 1..=3 {
                    let input = &a[fixed..fixed + k];
                    for exact in [true, false] {
                        let p = chain
                            .build_pattern(Some(input), None, None, exact)
                            .unwrap_or_else(|e| panic!("{id} '{input}' (from {a}): {e}"));
                        assert!(chain.expected_attempts(&p).is_finite(), "{id} '{input}'");
                        // `15` on Bitcoin may fairly be read as lead `1` + `5`.
                        if p.prefix.len() == fixed + k {
                            assert!(
                                chain.matches(a, &p),
                                "{id} '{input}' → '{}' misses {a}",
                                p.prefix
                            );
                        }
                        // Typing the full start is never ambiguous.
                        let full = &a[..fixed + k];
                        if let Ok(p) = chain.build_pattern(Some(full), None, None, exact) {
                            assert!(
                                chain.matches(a, &p),
                                "{id} '{full}' → '{}' misses {a}",
                                p.prefix
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn deploy_modes_resolve_and_roundtrip() {
        for (alias, id) in [
            ("evm-contract", "evm-contract"),
            ("contract", "evm-contract"),
            ("sol-mint", "sol-mint"),
            ("mint", "sol-mint"),
        ] {
            let chain = Chain::from_id(alias).unwrap_or_else(|e| panic!("{alias}: {e}"));
            assert_eq!(chain.id(), id);
            let (addr, attempt) = chain.grind_attempt();
            assert_eq!(chain.finalize(attempt).address, addr);
        }
    }
}
