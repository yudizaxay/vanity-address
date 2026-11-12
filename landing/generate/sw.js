// Offline copy of this page's own files. Never sees keys: it only stores static assets fetched from this site.
const CACHE = "vanity-generate-v41";

const ICONS = [
  "ada", "algo", "apt", "atom", "bch", "btc", "btg", "cdn", "ckb", "dash", "dcr", "dgb", "doge", "dot", "dydx",
  "egld", "emc2", "eth", "fil", "hbar", "icp", "inj", "kas", "ksm", "ltc", "mona", "near", "nvc", "osmo", "ppc",
  "qtum", "rvn", "sei", "sol", "stx", "sui", "sys", "tia", "ton", "trx", "via", "vtc", "xlm", "xpm", "xrp", "xtz",
  "zec",
];

const EXPLORER_ICONS = [
  "allo", "aptos", "arbiscan", "basescan", "blockchair", "blockstream", "bscscan", "cardanoscan", "etherscan",
  "filfox", "litecoinspace", "mempool", "mintscan", "multiversx", "nearblocks", "optimism", "polygonscan", "solana",
  "solscan", "stellarexpert", "subscan", "suiscan", "suivision", "tonviewer", "tronscan", "tzkt", "xrpscan",
];

const PRECACHE = [
  "./",
  "app.css",
  "app.js",
  "chain-meta.js",
  "theme.js",
  "worker.js",
  "manifest.webmanifest",
  "wasm/vanity_wasm.js",
  "wasm/vanity_wasm_bg.wasm",
  "app-icons/icon-192.png",
  "app-icons/icon-512.png",
  "app-icons/apple-touch-180.png",
  "grind-bot.png",
  "grind-bot-dark.png",
  "../assets/logo.svg",
  ...ICONS.map((t) => `icons/${t}.png`),
  ...EXPLORER_ICONS.map((n) => `explorers/${n}.png`),
];

self.addEventListener("install", (event) => {
  event.waitUntil(
    caches.open(CACHE).then((cache) => cache.addAll(PRECACHE)).then(() => self.skipWaiting()),
  );
});

self.addEventListener("activate", (event) => {
  event.waitUntil(
    caches
      .keys()
      .then((keys) => Promise.all(keys.filter((k) => k !== CACHE).map((k) => caches.delete(k))))
      .then(() => self.clients.claim()),
  );
});

// Network first so an online visitor always runs the latest code; the cache is only the offline fallback.
self.addEventListener("fetch", (event) => {
  const { request } = event;
  if (request.method !== "GET" || new URL(request.url).origin !== self.location.origin) return;
  event.respondWith(fromNetworkElseCache(request));
});

async function fromNetworkElseCache(request) {
  const cache = await caches.open(CACHE);
  const key = request.mode === "navigate" ? "./" : request;
  try {
    const res = await fetch(request);
    if (res.ok && res.type === "basic") cache.put(key, res.clone());
    return res;
  } catch {
    const cached = await cache.match(key, { ignoreSearch: request.mode === "navigate" });
    if (cached) return cached;
    throw new Error("offline and not cached");
  }
}
