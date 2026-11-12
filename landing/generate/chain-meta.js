// Display-only details for the chain picker. Chain ids and names still come from the engine.

// Shown first in the picker; everything else follows alphabetically.
export const POPULAR = ["sol", "evm", "btc", "ton", "trx", "sui"];
export const DEFAULT_CHAIN = "sol";
export const DEFAULT_DEPLOY = "evm-contract";
export const DEPLOY_POPULAR = ["evm-contract", "trx-contract", "sol-mint", "create2"];

// ticker, brand colour (color2 for two-tone logos, mono for black-and-white logos), and extra words people search for
export const META = {
  algo: { ticker: "ALGO", color: "#9aa3ad", mono: true, words: "algorand" },
  aptos: { ticker: "APT", color: "#3dd6c6", words: "aptos move" },
  btc: { ticker: "BTC", color: "#f7931a", words: "bitcoin legacy p2pkh 1" },
  bch: { ticker: "BCH", color: "#0ac18e", words: "bitcoin cash cashaddr" },
  btg: { ticker: "BTG", color: "#eba809", words: "bitcoin gold p2pkh" },
  "btc-segwit": { ticker: "BTC", color: "#f7931a", words: "bitcoin segwit bc1q native" },
  "btc-taproot": { ticker: "BTC", color: "#f7931a", words: "bitcoin taproot bc1p" },
  cdn: { ticker: "CDN", color: "#c8102e", words: "canada ecoin" },
  ada: { ticker: "ADA", color: "#2f6bff", words: "cardano addr1" },
  tia: { ticker: "TIA", color: "#8b46ff", words: "celestia" },
  cosmos: { ticker: "ATOM", color: "#8c8fb0", words: "cosmos hub atom" },
  dash: { ticker: "DASH", color: "#1c8de3", words: "dash" },
  dcr: { ticker: "DCR", color: "#2ed6a1", words: "decred ds p2pkh" },
  dgb: { ticker: "DGB", color: "#0066cc", words: "digibyte" },
  doge: { ticker: "DOGE", color: "#c9a93a", words: "dogecoin" },
  dydx: { ticker: "DYDX", color: "#7a77ff", words: "dydx" },
  emc2: { ticker: "EMC2", color: "#4fa9d4", words: "einsteinium" },
  evm: {
    ticker: "ETH",
    color: "#7b8ff0",
    words:
      "ethereum metamask base arbitrum optimism polygon bnb bsc avalanche hyperliquid sonic robinhood linea scroll zksync blast monad berachain",
  },
  fil: { ticker: "FIL", color: "#1a9bff", words: "filecoin" },
  hedera: { ticker: "HBAR", color: "#9a8cff", mono: true, words: "hedera hashgraph" },
  inj: { ticker: "INJ", color: "#5141ff", color2: "#1d8cff", words: "injective" },
  icp: { ticker: "ICP", color: "#ed1e79", color2: "#29abe2", words: "internet computer dfinity principal" },
  kaspa: { ticker: "KAS", color: "#49eacb", words: "kaspa" },
  ksm: { ticker: "KSM", color: "#e8026d", mono: true, words: "kusama" },
  ltc: { ticker: "LTC", color: "#6f8fc7", mono: true, words: "litecoin" },
  mona: { ticker: "MONA", color: "#b6a06c", words: "monacoin" },
  erd: { ticker: "EGLD", color: "#23f7dd", words: "multiversx elrond" },
  near: { ticker: "NEAR", color: "#00c08b", words: "near protocol" },
  ckb: { ticker: "CKB", color: "#3cb98c", words: "nervos ckb ckb1" },
  nvc: { ticker: "NVC", color: "#8b8b00", words: "novacoin" },
  osmo: { ticker: "OSMO", color: "#a24bd8", color2: "#e2047a", words: "osmosis" },
  ppc: { ticker: "PPC", color: "#3cb054", words: "peercoin" },
  dot: { ticker: "DOT", color: "#e6007a", mono: true, words: "polkadot" },
  xpm: { ticker: "XPM", color: "#f7931a", words: "primecoin" },
  qtum: { ticker: "QTUM", color: "#2e9ad0", words: "qtum" },
  rvn: { ticker: "RVN", color: "#384182", words: "ravencoin raven" },
  xrp: { ticker: "XRP", color: "#a3b1c2", mono: true, words: "ripple xrpl" },
  sei: { ticker: "SEI", color: "#c8433c", words: "sei" },
  sol: { ticker: "SOL", color: "#a066ff", color2: "#14f195", words: "solana phantom solflare backpack" },
  stx: { ticker: "STX", color: "#5546ff", words: "stacks sp c32" },
  xlm: { ticker: "XLM", color: "#b3b0ff", mono: true, words: "stellar lumens" },
  sui: { ticker: "SUI", color: "#4da2ff", words: "sui move" },
  sys: { ticker: "SYS", color: "#0082c6", words: "syscoin" },
  xtz: { ticker: "XTZ", color: "#2c7df7", words: "tezos" },
  ton: { ticker: "TON", color: "#0098ea", words: "toncoin telegram tonkeeper" },
  trx: { ticker: "TRX", color: "#eb0029", words: "tron tronlink" },
  vtc: { ticker: "VTC", color: "#1b5c2e", words: "vertcoin" },
  via: { ticker: "VIA", color: "#565656", mono: true, words: "viacoin" },
  zec: { ticker: "ZEC", color: "#f4b728", words: "zcash transparent t1" },
  // Deploy / mint modes (Contract toggle) — logos reuse wallet tickers
  "evm-contract": {
    ticker: "ETH",
    color: "#7b8ff0",
    words: "evm contract create deployer nonce cafe ethereum",
  },
  "trx-contract": {
    ticker: "TRX",
    color: "#eb0029",
    words: "tron contract create deployer nonce cafe",
  },
  "sol-mint": {
    ticker: "SOL",
    color: "#a066ff",
    color2: "#14f195",
    words: "solana mint spl token pump",
  },
  create2: {
    ticker: "ETH",
    color: "#7b8ff0",
    words: "create2 salt factory init code hash ethereum contract",
  },
};

// Public block explorers for checking a result: [name, icon path, address URL prefix].
// Hedera has none: its address is a public key and the 0.0.N account id is assigned on-chain.
const MEMPOOL = ["mempool.space", "explorers/mempool.png", "https://mempool.space/address/"];
const BLOCKSTREAM = ["Blockstream", "explorers/blockstream.png", "https://blockstream.info/address/"];
const mintscan = (net) => ["Mintscan", "explorers/mintscan.png", `https://www.mintscan.io/${net}/address/`];
const blockchair = (net) => ["Blockchair", "explorers/blockchair.png", `https://blockchair.com/${net}/address/`];

export const EXPLORERS = {
  evm: [
    ["Etherscan", "explorers/etherscan.png", "https://etherscan.io/address/"],
    ["Basescan", "explorers/basescan.png", "https://basescan.org/address/"],
    ["BscScan", "explorers/bscscan.png", "https://bscscan.com/address/"],
    ["Arbiscan", "explorers/arbiscan.png", "https://arbiscan.io/address/"],
    ["Polygonscan", "explorers/polygonscan.png", "https://polygonscan.com/address/"],
    ["Optimism", "explorers/optimism.png", "https://optimistic.etherscan.io/address/"],
  ],
  sol: [
    ["Solscan", "explorers/solscan.png", "https://solscan.io/account/"],
    ["Solana Explorer", "explorers/solana.png", "https://explorer.solana.com/address/"],
  ],
  btc: [MEMPOOL, BLOCKSTREAM],
  "btc-segwit": [MEMPOOL, BLOCKSTREAM],
  "btc-taproot": [MEMPOOL, BLOCKSTREAM],
  ltc: [["Litecoin Space", "explorers/litecoinspace.png", "https://litecoinspace.org/address/"], blockchair("litecoin")],
  doge: [blockchair("dogecoin")],
  dash: [blockchair("dash")],
  bch: [["Blockchair", "explorers/blockchair.png", "https://blockchair.com/bitcoin-cash/address/"]],
  dcr: [["dcrdata", "icons/dcr.png", "https://mainnet.dcrdata.org/address/"]],
  stx: [["Hiro Explorer", "icons/stx.png", "https://explorer.hiro.so/address/"]],
  zec: [["Blockchair", "explorers/blockchair.png", "https://blockchair.com/zcash/address/"]],
  ckb: [["Nervos Explorer", "icons/ckb.png", "https://explorer.nervos.org/address/"]],
  ada: [["Cardanoscan", "explorers/cardanoscan.png", "https://cardanoscan.io/address/"]],
  cosmos: [mintscan("cosmos")],
  tia: [mintscan("celestia")],
  dydx: [mintscan("dydx")],
  osmo: [mintscan("osmosis")],
  inj: [mintscan("injective")],
  sei: [mintscan("sei")],
  trx: [["Tronscan", "explorers/tronscan.png", "https://tronscan.org/#/address/"]],
  "evm-contract": [
    ["Etherscan", "explorers/etherscan.png", "https://etherscan.io/address/"],
    ["Basescan", "explorers/basescan.png", "https://basescan.org/address/"],
  ],
  "trx-contract": [["Tronscan", "explorers/tronscan.png", "https://tronscan.org/#/address/"]],
  "sol-mint": [
    ["Solscan", "explorers/solscan.png", "https://solscan.io/token/"],
    ["Solana Explorer", "explorers/solana.png", "https://explorer.solana.com/address/"],
  ],
  create2: [
    ["Etherscan", "explorers/etherscan.png", "https://etherscan.io/address/"],
    ["Basescan", "explorers/basescan.png", "https://basescan.org/address/"],
  ],
  ton: [["Tonviewer", "explorers/tonviewer.png", "https://tonviewer.com/"]],
  sui: [
    ["Suiscan", "explorers/suiscan.png", "https://suiscan.xyz/mainnet/account/"],
    ["SuiVision", "explorers/suivision.png", "https://suivision.xyz/account/"],
  ],
  aptos: [["Aptos Explorer", "explorers/aptos.png", "https://explorer.aptoslabs.com/account/"]],
  near: [["NearBlocks", "explorers/nearblocks.png", "https://nearblocks.io/address/"]],
  dot: [["Subscan", "explorers/subscan.png", "https://polkadot.subscan.io/account/"]],
  ksm: [["Subscan", "explorers/subscan.png", "https://kusama.subscan.io/account/"]],
  xrp: [["XRPScan", "explorers/xrpscan.png", "https://xrpscan.com/account/"]],
  xlm: [["StellarExpert", "explorers/stellarexpert.png", "https://stellar.expert/explorer/public/account/"]],
  algo: [["Allo", "explorers/allo.png", "https://allo.info/account/"]],
  xtz: [["TzKT", "explorers/tzkt.png", "https://tzkt.io/"]],
  fil: [["Filfox", "explorers/filfox.png", "https://filfox.info/en/address/"]],
  kaspa: [["Kaspa Explorer", "icons/kas.png", "https://explorer.kaspa.org/addresses/"]],
  erd: [["MultiversX Explorer", "explorers/multiversx.png", "https://explorer.multiversx.com/accounts/"]],
};

// "Bitcoin SegWit (bc1q · P2WPKH)" -> { title: "Bitcoin SegWit", detail: "bc1q · P2WPKH" }
export function splitLabel(label) {
  const m = /^(.*?)\s*\((.*)\)\s*$/.exec(label);
  return m ? { title: m[1], detail: m[2] } : { title: label, detail: "" };
}
