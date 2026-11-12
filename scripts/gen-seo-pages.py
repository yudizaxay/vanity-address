#!/usr/bin/env python3
"""Generate landing/ SEO pages: per-chain guides, /chains/ hub, difficulty calculator, sitemap.xml.

Run from repo root:  python3 scripts/gen-seo-pages.py
Output is committed (Cloudflare Pages serves landing/ as-is).
"""
import html
import json
import math
import pathlib

ROOT = pathlib.Path(__file__).resolve().parent.parent / "landing"
SITE = "https://vanity-address.online"
TODAY = "2026-11-12"
OG = f"{SITE}/assets/og.png"
FONTS = "https://fonts.googleapis.com/css2?family=Fraunces:opsz,wght@9..144,700&family=Source+Sans+3:wght@400;600;700&family=IBM+Plex+Mono:wght@400;500&display=swap"
GH = "https://github.com/yudizaxay/vanity-address"

# Existing hand-written pages (kept in sitemap + related links).
GUIDES = [
    ("solana-vanity-address", "Solana vanity address"),
    ("ethereum-vanity-address", "Ethereum / EVM vanity address"),
    ("bitcoin-vanity-address", "Bitcoin vanity address"),
    ("create2-vanity-address", "CREATE2 vanity contract"),
]

B58 = ("Base58", 58)
CHAINS = [
    dict(slug="tron-vanity-address", id="trx", name="Tron", tick="TRX", alpha=B58, ex="TLove…", sfx="Love",
         title="Tron vanity address generator (offline, TRX)",
         desc="Generate a custom Tron (TRX) wallet address offline with your own prefix or suffix. Free, open source, keys never leave your device.",
         about=["Tron addresses are 34-character Base58Check strings that always begin with <code>T</code>. Because the first character is fixed, your custom pattern effectively starts at the second character, or you can anchor it at the end with a suffix.",
                "The Base58 alphabet leaves out <code>0</code>, <code>O</code>, <code>I</code> and <code>l</code> so addresses are easy to read aloud. Patterns containing those characters can never match."],
         tips=["Suffixes such as repeated digits (<code>8888</code>) or a short word read well on explorers like Tronscan.",
               "Tron token contracts have a separate mode: <code>--chain trx-contract</code> grinds the contract address a deployer will produce.",
               "Case matters in Base58, so <code>Love</code> and <code>love</code> are different patterns with different odds."],
         faq=[("Does every Tron address start with T?", "Yes. Mainnet Tron addresses use version byte 0x41, which always encodes to a leading T, so that character cannot be customised."),
              ("How long does a 4-character Tron suffix take?", "On average about 58⁴ ≈ 11.3 million attempts. Divide by your keys-per-second to estimate time; the difficulty calculator does this for you."),
              ("Can I make a Tron vanity contract address?", "Yes, use the trx-contract mode in the CLI, desktop app or web generator.")]),
    dict(slug="dogecoin-vanity-address", id="doge", name="Dogecoin", tick="DOGE", alpha=B58, ex="DDoge…", sfx="Doge",
         title="Dogecoin vanity address generator (offline, DOGE)",
         desc="Make a custom Dogecoin address offline — pick your own prefix or suffix. Free, open source, runs locally in your browser, CLI, or desktop app.",
         about=["Dogecoin uses Bitcoin-style P2PKH addresses with a different version byte, so every address begins with <code>D</code> and is 34 characters of Base58Check.",
                "vanity-address grinds secp256k1 keys locally and re-derives the address before showing it, so what you see is exactly what a wallet will import."],
         tips=["Import the exported WIF private key into a Dogecoin wallet that supports importing keys.",
               "Case-sensitive patterns are exact: <code>Doge</code> is much rarer than any-case <em>doge</em>.",
               "Never reuse a vanity key as a seed phrase wallet — it is a single raw key, so back it up separately."],
         faq=[("Why do Dogecoin addresses start with D?", "The P2PKH version byte 0x1E encodes to a leading D in Base58Check."),
              ("Is a Dogecoin vanity address less secure?", "No, as long as the key comes from a proper random source. The pattern only constrains which key you keep, not how it was generated."),
              ("What is the easiest Dogecoin pattern?", "One or two characters take under a second. Each extra character multiplies work by about 58.")]),
    dict(slug="litecoin-vanity-address", id="ltc", name="Litecoin", tick="LTC", alpha=B58, ex="LLite…", sfx="Lite",
         title="Litecoin vanity address generator (offline, LTC)",
         desc="Create a custom Litecoin address offline with your own prefix or suffix. Free, open source, keys never leave your device.",
         about=["Litecoin legacy P2PKH addresses are 34-character Base58Check strings starting with <code>L</code>. vanity-address generates these locally from a fresh random secp256k1 key.",
                "Because the alphabet is Base58, characters <code>0 O I l</code> are invalid in patterns."],
         tips=["Use a suffix for a branded look; a prefix after the leading L is equally possible but harder to read on small screens.",
               "Verify any grind result with <code>vanity-address verify</code> before moving funds.",
               "For large balances prefer the offline CLI or desktop app on a clean machine."],
         faq=[("Do Litecoin addresses always start with L?", "Legacy P2PKH addresses do. P2SH addresses start with M and SegWit ones with ltc1."),
              ("How hard is a 5-character Litecoin pattern?", "About 58⁵ ≈ 656 million attempts on average.")]),
    dict(slug="dash-vanity-address", id="dash", name="Dash", tick="DASH", alpha=B58, ex="XDash…", sfx="Dash",
         title="Dash vanity address generator (offline)",
         desc="Generate a custom Dash address offline with a prefix or suffix of your choice. Free, open source, and fully local.",
         about=["Dash P2PKH addresses are 34-character Base58Check strings that begin with <code>X</code>. The first character is fixed by the network version byte.",
                "The generator runs entirely on your device; the private key is exported as WIF and never transmitted."],
         tips=["Keep patterns short — four characters is already tens of millions of attempts.",
               "Case-sensitive patterns are rarer; ignoring case roughly doubles your odds for letter-heavy words."],
         faq=[("Why do Dash addresses start with X?", "The version byte 0x4C maps to a leading X in Base58Check."),
              ("Can I choose the first character?", "No. Anything after the leading X is customisable.")]),
    dict(slug="ton-vanity-address", id="ton", name="TON", tick="TON", alpha=("Base64url", 64), ex="UQ…", sfx="moon",
         title="TON vanity address generator (offline, Wallet V4R2)",
         desc="Generate a custom TON wallet address offline (V4R2) with your own prefix or suffix. Free, open source, keys stay on your device.",
         about=["TON user-friendly addresses are 48-character base64url strings. The first two characters encode flags and workchain, and the third character can only be one of <code>A–D</code> for workchain 0, so only part of the front of the address is free.",
                "vanity-address computes the Wallet V4R2 address from the public key and the contract code hash, then CRC-checks the result before showing it."],
         tips=["Suffix patterns are the most flexible on TON because the end of the address is fully random.",
               "The base64url alphabet allows <code>A–Z a–z 0–9 _ -</code> and is case-sensitive.",
               "Import the exported key into a wallet that supports V4R2 to get the same address."],
         faq=[("Why can't I set the first characters of a TON address?", "They carry address flags and workchain, so only certain values are valid. The tool accounts for those fixed bits when estimating difficulty."),
              ("How many attempts does a 4-character TON suffix need?", "About 64⁴ ≈ 16.8 million on average.")]),
    dict(slug="sui-vanity-address", id="sui", name="Sui", tick="SUI", alpha=("Hex", 16), ex="0xdead…", sfx="dead",
         title="Sui vanity address generator (offline)",
         desc="Make a custom Sui address starting or ending with your own hex pattern. Offline, free, open source.",
         about=["A Sui address is <code>0x</code> followed by 64 hex characters, derived by hashing the Ed25519 public key with a signature-scheme flag using Blake2b.",
                "Hex has only 16 symbols per position, so Sui patterns are much cheaper than Base58 ones of the same length. Only <code>0–9</code> and <code>a–f</code> are valid."],
         tips=["Words made from a–f work: <code>dead</code>, <code>cafe</code>, <code>face</code>, <code>fade</code>, <code>decade</code>.",
               "Digits can stand in for letters (<code>5</code> for S, <code>1</code> for I) to extend your vocabulary.",
               "Export the key and import it into a Sui wallet; the address should match."],
         faq=[("Which letters can a Sui vanity pattern use?", "Only hex digits: 0-9 and a-f."),
              ("How long does a 6-hex-character Sui prefix take?", "16⁶ ≈ 16.8 million attempts on average — a few seconds to minutes depending on hardware.")]),
    dict(slug="near-vanity-address", id="near", name="NEAR", tick="NEAR", alpha=("Hex", 16), ex="cafe…", sfx="cafe",
         title="NEAR vanity address generator (implicit account, offline)",
         desc="Generate a NEAR implicit account with a custom hex prefix or suffix offline. Free, open source, keys never leave your device.",
         about=["NEAR implicit accounts are the 64-character lowercase hex encoding of an Ed25519 public key. Because the account ID is the key, you can fund it before ever creating it on-chain.",
                "Patterns are limited to <code>0–9</code> and <code>a–f</code>. Named accounts such as <em>alice.near</em> are registered on-chain and cannot be ground."],
         tips=["Short hex words like <code>dead</code> or <code>cafe</code> take seconds.",
               "Each extra hex character multiplies the work by 16 — one more character is 16× slower.",
               "Back up the exported key before sending funds to the account."],
         faq=[("Is this a named .near account?", "No, it is an implicit account whose ID is the hex public key."),
              ("What is the expected time for an 8-character hex prefix?", "16⁸ ≈ 4.3 billion attempts on average.")]),
    dict(slug="xrp-vanity-address", id="xrp", name="XRP Ledger", tick="XRP", alpha=("Ripple Base58", 58), ex="rMoon…", sfx="moon",
         title="XRP (Ripple) vanity address generator (offline)",
         desc="Generate a custom XRP Ledger classic address offline with your own prefix or suffix. Free, open source, keys never leave your device.",
         about=["XRP classic addresses start with <code>r</code> and use the Ripple Base58 alphabet, which is ordered differently from Bitcoin's. It omits <code>0 O I l</code> and also the uppercase letters <code>A C F</code>.",
                "vanity-address uses the Ripple alphabet when checking patterns, so a pattern that is valid for Bitcoin may be impossible on XRP."],
         tips=["Lowercase words usually work well — <code>moon</code>, <code>rich</code>, <code>dao</code>.",
               "Confirm results on XRPSCAN or in any wallet that imports family seeds before funding.",
               "Remember the XRP Ledger reserve requirement when activating a new address."],
         faq=[("Why is my XRP pattern rejected?", "It likely contains a character outside the Ripple alphabet, such as 0, O, I, l, A, C or F."),
              ("Do XRP addresses always start with r?", "Yes, classic addresses begin with r.")]),
    dict(slug="stellar-vanity-address", id="xlm", name="Stellar", tick="XLM", alpha=("Base32", 32), ex="G…MOON", sfx="MOON",
         title="Stellar (XLM) vanity address generator (offline)",
         desc="Make a custom Stellar G… address offline with your own prefix or suffix. Free, open source, keys never leave your device.",
         about=["Stellar public keys are 56-character StrKey strings that start with <code>G</code>, encoded in uppercase RFC 4648 Base32 (<code>A–Z</code> and <code>2–7</code>).",
                "The alphabet has 32 symbols and no lowercase, so patterns must be uppercase and cannot contain <code>0 1 8 9</code>."],
         tips=["Suffix words in capitals look best: <code>MOON</code>, <code>STAR</code>, <code>DEFI</code>.",
               "A new Stellar account needs a minimum XLM balance to activate on-chain.",
               "Export the secret (starting with S) and keep it offline."],
         faq=[("Can Stellar vanity patterns include lowercase letters?", "No. StrKey is uppercase Base32 only."),
              ("How many attempts for a 5-character Stellar suffix?", "32⁵ ≈ 33.5 million on average.")]),
    dict(slug="cardano-vanity-address", id="ada", name="Cardano", tick="ADA", alpha=("Bech32", 32), ex="addr1v…cafe", sfx="cafe",
         title="Cardano vanity address generator (offline, Shelley)",
         desc="Generate a custom Cardano Shelley address offline with a bech32 prefix or suffix. Free, open source, local-only.",
         about=["Cardano Shelley enterprise addresses are bech32 strings starting with <code>addr1</code>. After that comes a header character, which for an enterprise mainnet address is always <code>v</code>, then one of <code>y 9 x 8</code>.",
                "Bech32 excludes <code>1 b i o</code>, so words containing those letters cannot match. The remaining 32 characters are <code>qpzry9x8gf2tvdw0s3jn54khce6mua7l</code>."],
         tips=["Words that fit the bech32 set include <code>cafe</code>, <code>dead</code>, <code>fade</code> and <code>defa</code>; anything with <code>b</code>, <code>i</code>, <code>o</code> or <code>1</code> (like <em>moon</em>) cannot match.",
               "This generates payment-key-only enterprise addresses, not staking-enabled base addresses.",
               "Always test with a small amount first."],
         faq=[("Does the tool create staking addresses?", "No — enterprise addresses only (payment key, no stake key)."),
              ("Which letters are not allowed?", "In the data part, 1, b, i and o are not in the bech32 alphabet.")]),
    dict(slug="kaspa-vanity-address", id="kaspa", name="Kaspa", tick="KAS", alpha=("Bech32", 32), ex="kaspa:q…", sfx="dead",
         title="Kaspa vanity address generator (offline)",
         desc="Make a custom Kaspa address offline with your own prefix or suffix. Free, open source, keys never leave your device.",
         about=["Kaspa addresses look like <code>kaspa:q…</code> and use a bech32-style charset. For version-0 public-key addresses the first data character is one of <code>q p z r</code>.",
                "vanity-address knows which leading characters are reachable and shows an honest ETA instead of silently searching an impossible prefix."],
         tips=["Suffix patterns avoid the restricted leading characters entirely.",
               "Valid pattern characters: <code>qpzry9x8gf2tvdw0s3jn54khce6mua7l</code>.",
               "Check the address on a Kaspa explorer before sending."],
         faq=[("Why are some Kaspa prefixes impossible?", "The first data character carries version bits, so only q, p, z or r can appear first."),
              ("Is the key stored anywhere?", "No. It exists only in memory and in the file you choose to export.")]),
    dict(slug="zcash-vanity-address", id="zec", name="Zcash", tick="ZEC", alpha=B58, ex="t1Moon…", sfx="Moon",
         title="Zcash vanity address generator (transparent t1, offline)",
         desc="Generate a custom Zcash transparent (t1) address offline with your own prefix or suffix. Free, open source, keys never leave your device.",
         about=["Zcash transparent P2PKH addresses are Base58Check strings that begin with <code>t1</code>. The network uses a two-byte version prefix, so the leading <code>t1</code> is fixed and your pattern starts after that.",
                "Shielded Sapling (<code>zs1…</code>) and Unified addresses are out of scope — this mode grinds transparent secp256k1 keys only."],
         tips=["Suffixes such as short words or repeated digits read well on explorers.",
               "Import the exported WIF or hex key into a wallet that supports transparent Zcash keys.",
               "Case matters in Base58, so <code>Moon</code> and <code>moon</code> are different patterns."],
         faq=[("Do Zcash addresses always start with t1?", "Transparent P2PKH mainnet addresses do. Shielded and Unified formats use other prefixes and are not generated here."),
              ("Is a transparent vanity address private?", "No. Transparent addresses are visible on-chain like Bitcoin. Use shielded wallets if you need privacy.")]),
    dict(slug="bitcoin-cash-vanity-address", id="bch", name="Bitcoin Cash", tick="BCH", alpha=("CashAddr", 32), ex="q…cafe", sfx="cafe",
         title="Bitcoin Cash vanity address generator (CashAddr, offline)",
         desc="Make a custom Bitcoin Cash CashAddr (q…) offline with your own prefix or suffix. Free, open source, fully local.",
         about=["Bitcoin Cash P2PKH CashAddr addresses use the bech32-style charset and typically begin with <code>q</code> (or <code>bitcoincash:q</code> with the HRP).",
                "The engine grinds the same secp256k1 + hash160 as Bitcoin, then encodes CashAddr instead of legacy Base58."],
         tips=["Patterns use the CashAddr alphabet: <code>qpzry9x8gf2tvdw0s3jn54khce6mua7l</code>.",
               "Words with <code>b</code>, <code>i</code>, <code>o</code> or <code>1</code> cannot match.",
               "Export WIF and import into any BCH wallet that accepts Bitcoin-style keys."],
         faq=[("Is this legacy Base58 BCH?", "No — CashAddr only (q…). Legacy 1… addresses are not generated."),
              ("How hard is a 4-character CashAddr suffix?", "About 32⁴ ≈ 1 million attempts on average.")]),
    dict(slug="stacks-vanity-address", id="stx", name="Stacks", tick="STX", alpha=("c32", 32), ex="SP…MOON", sfx="MOON",
         title="Stacks vanity address generator (SP…, offline)",
         desc="Generate a custom Stacks (STX) SP… address offline with your own prefix or suffix. Free, open source, keys stay on your device.",
         about=["Stacks mainnet singlesig addresses start with <code>SP</code> and use Crockford base32 (c32check) over hash160 of a compressed secp256k1 public key.",
                "The alphabet is uppercase <code>0–9 A–Z</code> without <code>I L O U</code>, so patterns must fit that set."],
         tips=["Prefer uppercase suffixes that avoid I, L, O and U.",
               "The same key can also derive a linked Bitcoin P2PKH address.",
               "Import hex or WIF into Hiro / Leather / Xverse-compatible wallets."],
         faq=[("Why do Stacks addresses start with SP?", "The leading S marks c32check, and version 0x16 encodes to P for mainnet singlesig."),
              ("Can I use lowercase patterns?", "c32 is uppercase-only; the tool normalises case where appropriate.")]),
    dict(slug="decred-vanity-address", id="dcr", name="Decred", tick="DCR", alpha=B58, ex="DsMoon…", sfx="Moon",
         title="Decred vanity address generator (Ds…, offline)",
         desc="Create a custom Decred (DCR) Ds… address offline with your own prefix or suffix. Free, open source, local-only.",
         about=["Decred mainnet P2PKH addresses begin with <code>Ds</code>. Unlike Bitcoin, Decred uses BLAKE256 for hash160 and a double-BLAKE256 Base58Check checksum with a two-byte version.",
                "vanity-address re-derives every candidate on the CPU before export so the address matches Decrediton / dcrwallet import."],
         tips=["The leading <code>Ds</code> is fixed — start your pattern after it, or use a suffix.",
               "Export hex or Decred WIF (<code>Pm…</code>) for wallet import.",
               "Case-sensitive Base58 patterns are exact; ignoring case roughly doubles odds for letter-heavy words."],
         faq=[("Why Ds and not D?", "Mainnet P2PKH uses a two-byte version that Base58-encodes to Ds."),
              ("Does Decred use SHA-256 like Bitcoin?", "No. Address hashing uses BLAKE256 + RIPEMD160.")]),
    dict(slug="nervos-ckb-vanity-address", id="ckb", name="Nervos CKB", tick="CKB", alpha=("Bech32", 32), ex="ckb1qyq…cafe", sfx="cafe",
         title="Nervos CKB vanity address generator (ckb1qyq…, offline)",
         desc="Generate a custom Nervos CKB short address offline with your own prefix or suffix. Free, open source, keys never leave your device.",
         about=["Short secp256k1 CKB addresses look like <code>ckb1qyq…</code>. The payload is format <code>0x01</code>, code hash index <code>0x00</code>, then blake160 of the compressed public key (Blake2b with personalization <code>ckb-default-hash</code>).",
                "Bech32 excludes <code>1 b i o</code> in the data part, so choose patterns from the remaining 32 characters."],
         tips=["The fixed head <code>ckb1qyq</code> is applied for you — put custom characters after it or use a suffix.",
               "Short format is widely supported; newer full Bech32m lock-script addresses are out of scope here.",
               "Import the hex private key into Neuron or another CKB wallet."],
         faq=[("Why ckb1qyq?", "Short format type 0x01 and secp index 0x00 encode to those leading data characters after the HRP."),
              ("Are full CKB addresses supported?", "Not yet — this release grinds the popular short secp256k1 form.")]),
]

CSS_PAGE = '<link rel="stylesheet" href="../assets/guide.css" />'


def e(s):
    return html.escape(s, quote=True)


def fmt_num(n):
    if n < 1e6:
        return f"{n:,.0f}"
    for div, name in ((1e15, "quadrillion"), (1e12, "trillion"), (1e9, "billion"), (1e6, "million")):
        if n >= div:
            v = n / div
            return f"{v:,.1f} {name}" if v < 100 else f"{v:,.0f} {name}"
    return str(n)


def fmt_time(sec):
    if sec < 1:
        return "under a second"
    units = [("year", 31557600), ("day", 86400), ("hour", 3600), ("minute", 60), ("second", 1)]
    for name, s in units:
        if sec >= s:
            v = sec / s
            v = round(v, 1) if v < 10 else round(v)
            if v >= 1000 and name == "year":
                return f"{v:,.0f} years"
            return f"{v:g} {name}{'' if v == 1 else 's'}"
    return "under a second"


def diff_table(base, speed=1_000_000, rows=range(1, 9)):
    out = ['<table class="diff"><thead><tr><th>Pattern length</th><th>Expected attempts</th><th>Average time @ 1M keys/s</th></tr></thead><tbody>']
    for n in rows:
        a = base ** n
        out.append(f"<tr><td>{n}</td><td>{fmt_num(a)}</td><td>{fmt_time(a / speed)}</td></tr>")
    out.append("</tbody></table>")
    return "\n".join(out)


def head(title, desc, path, og_type="article", extra=""):
    url = f"{SITE}/{path}"
    return f'''<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8" />
<meta name="viewport" content="width=device-width, initial-scale=1.0" />
<title>{e(title)}</title>
<meta name="description" content="{e(desc)}" />
<link rel="canonical" href="{url}" />
<meta name="robots" content="index, follow, max-image-preview:large" />
<meta property="og:type" content="{og_type}" />
<meta property="og:site_name" content="vanity-address" />
<meta property="og:url" content="{url}" />
<meta property="og:title" content="{e(title)}" />
<meta property="og:description" content="{e(desc)}" />
<meta property="og:image" content="{OG}" />
<meta property="og:image:width" content="1200" />
<meta property="og:image:height" content="630" />
<meta name="twitter:card" content="summary_large_image" />
<meta name="twitter:title" content="{e(title)}" />
<meta name="twitter:description" content="{e(desc)}" />
<meta name="twitter:image" content="{OG}" />
<link rel="icon" href="../assets/logo.svg" type="image/svg+xml" />
<link rel="preconnect" href="https://fonts.googleapis.com" />
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
<link href="{FONTS}" rel="stylesheet" />
{CSS_PAGE}
{extra}
'''


def jsonld(obj):
    return '<script type="application/ld+json">\n' + json.dumps(obj, indent=2, ensure_ascii=False) + "\n</script>\n"


def breadcrumb(items):
    return {
        "@type": "BreadcrumbList",
        "itemListElement": [
            {"@type": "ListItem", "position": i + 1, "name": n, "item": u} for i, (n, u) in enumerate(items)
        ],
    }


NAV_SCRIPT = '''<script>
(function () {
  var t = document.getElementById("navToggle");
  var l = document.getElementById("navLinks");
  if (!t || !l) return;
  t.addEventListener("click", function () {
    var open = l.classList.toggle("open");
    t.setAttribute("aria-expanded", open ? "true" : "false");
  });
})();
</script>
'''

# Same Octocat mark as landing/index.html nav CTA
GH_ICON = """<svg viewBox="0 0 16 16" width="15" height="15" fill="currentColor" aria-hidden="true"><path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.01 8.01 0 0 0 16 8c0-4.42-3.58-8-8-8z"/></svg>"""


def site_nav(root="../"):
    """Landing-matching sticky nav. root is '../' for nested pages or '/' for site root."""
    r = root if root.endswith("/") else root + "/"
    return f'''<header class="nav">
  <div class="nav-inner">
    <a class="brand" href="{r}"><img src="{r}assets/logo.svg" alt="" width="34" height="34" /><span>vanity-address</span></a>
    <nav class="nav-links" id="navLinks" aria-label="Site">
      <a href="{r}generate/">Web app</a>
      <a href="{r}#download">Download</a>
      <a href="{r}#how">How it works</a>
      <a href="{r}#deploy">Contracts</a>
      <a href="{r}#demo">Demo</a>
      <a href="{r}chains/">Chains</a>
      <a href="{r}vanity-address-difficulty/">Difficulty</a>
      <a href="{r}#install">Install</a>
      <a href="{r}#faq">FAQ</a>
      <a href="{r}#security">Security</a>
    </nav>
    <div class="nav-actions">
      <a class="nav-cta" href="{GH}" target="_blank" rel="noopener" aria-label="Star on GitHub">
        {GH_ICON}
        <span class="cta-text">Star on GitHub</span>
      </a>
      <button class="nav-toggle" id="navToggle" type="button" aria-label="Toggle menu" aria-expanded="false">
        <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2"><path d="M4 7h16M4 12h16M4 17h16"/></svg>
      </button>
    </div>
  </div>
</header>
'''


def shell(top, main, related_html, root="../"):
    # `top` kept for call-site compatibility; nav is always site_nav(root).
    _ = top
    return f'''</head>
<body>
{site_nav(root)}
  <div class="wrap">
    <main>
{main}
{related_html}
    </main>
    <footer class="foot">
      <p><a href="{root}">← vanity-address home</a> · <a href="{root}chains/">All supported chains</a> · MIT · <a href="{GH}">Source on GitHub</a> · Updated {TODAY}</p>
    </footer>
  </div>
{NAV_SCRIPT}
</body>
</html>
'''


def related(current):
    items = [(f"../{s}/", n) for s, n in GUIDES] + [(f"../{c['slug']}/", f"{c['name']} vanity address") for c in CHAINS]
    lis = "\n".join(f'          <li><a href="{u}">{e(n)}</a></li>' for u, n in items if u != f"../{current}/")
    return f'''      <aside class="related">
        <h2>More vanity address guides</h2>
        <ul>
{lis}
          <li><a href="../vanity-address-difficulty/">Vanity address difficulty calculator</a></li>
          <li><a href="../chains/">All 49 supported chains</a></li>
        </ul>
      </aside>'''


def chain_page(c):
    aname, asize = c["alpha"]
    path = f'{c["slug"]}/'
    faq_ld = {"@type": "FAQPage", "mainEntity": [
        {"@type": "Question", "name": q, "acceptedAnswer": {"@type": "Answer", "text": a}} for q, a in c["faq"]]}
    ld = {"@context": "https://schema.org", "@graph": [
        {"@type": "TechArticle", "headline": c["title"], "description": c["desc"], "url": f"{SITE}/{path}",
         "image": OG, "datePublished": TODAY, "dateModified": TODAY, "inLanguage": "en",
         "author": {"@type": "Person", "name": "yudizaxay", "url": "https://github.com/yudizaxay"},
         "publisher": {"@type": "Organization", "name": "vanity-address", "url": f"{SITE}/",
                       "logo": {"@type": "ImageObject", "url": f"{SITE}/assets/logo-512.png"}},
         "mainEntityOfPage": f"{SITE}/{path}"},
        breadcrumb([("Home", f"{SITE}/"), ("All chains", f"{SITE}/chains/"), (f'{c["name"]} vanity address', f"{SITE}/{path}")]),
        faq_ld]}
    body_about = "\n".join(f"      <p>{p}</p>" for p in c["about"])
    tips = "\n".join(f"        <li>{t}</li>" for t in c["tips"])
    faqs = "\n".join(f"      <h3>{e(q)}</h3>\n      <p>{e(a)}</p>" for q, a in c["faq"])
    main = f'''      <p class="kicker">{e(c["name"])} · {e(c["tick"])} · offline</p>
      <h1>{e(c["name"])} vanity address generator</h1>
      <p class="lede">Create a real {e(c["name"])} address that starts with, ends with, or contains your own pattern — generated on your machine. No accounts, no telemetry, keys never leave your device.</p>
      <div class="ctas">
        <a class="btn btn-primary" href="../generate/">Generate in browser</a>
        <a class="btn btn-ghost" href="{GH}/releases/latest">Download CLI / desktop</a>
      </div>

      <h2>How {e(c["name"])} addresses work</h2>
{body_about}
      <p>Alphabet: <strong>{e(aname)}</strong> ({asize} symbols per position). Typical shape: <code>{e(c["ex"])}</code>.</p>

      <h2>How to generate a {e(c["name"])} vanity address</h2>
      <ol>
        <li>Open the <a href="../generate/">web generator</a> or run <code>npx vanity-address</code>.</li>
        <li>Choose <strong>{e(c["name"])}</strong> (<code>{c["id"]}</code>).</li>
        <li>Enter a short prefix, suffix, or contains pattern using the {e(aname)} alphabet.</li>
        <li>Check the ETA, grind, then verify the result before using it.</li>
      </ol>
      <pre>npx vanity-address --chain {c["id"]} --suffix {c["sfx"]}</pre>

      <h2>How long will it take?</h2>
      <p>Each extra character multiplies the work by about {asize}. Times below are illustrative at one million keys per second; your device may be faster or slower. Use the <a href="../vanity-address-difficulty/">difficulty calculator</a> with your own speed.</p>
{diff_table(asize)}

      <h2>Tips for {e(c["name"])}</h2>
      <ul>
{tips}
      </ul>

      <h2>Safety</h2>
      <p>Whoever holds the private key controls the funds. vanity-address creates keys locally from your operating system's secure random source and re-derives every address on the CPU before export. The exported file is a plaintext secret: keep it offline and never commit or email it. Read the <a href="{GH}/blob/main/docs/SECURITY-MODEL.md">security model</a> before moving significant value.</p>

      <h2>Frequently asked questions</h2>
{faqs}'''
    page = head(c["title"], c["desc"], path) + jsonld(ld) + shell(None, main, related(c["slug"]))
    (ROOT / c["slug"]).mkdir(exist_ok=True)
    (ROOT / c["slug"] / "index.html").write_text(page)


def hub_page():
    path = "chains/"
    title = "Vanity address generator for 49 chains — all supported"
    desc = "Every blockchain vanity-address can grind: Solana, EVM, Bitcoin, Zcash, BCH, Stacks, Decred, Nervos CKB, Tron, TON, Sui and more. Offline, free, open source."
    items = [(f"../{s}/", n, "In-depth guide") for s, n in GUIDES] + [
        (f"../{c['slug']}/", f"{c['name']} ({c['tick']})", f"{c['alpha'][0]} · {c['alpha'][1]} symbols") for c in CHAINS]
    lis = "\n".join(f'        <li><a href="{u}">{e(n)} vanity address</a> — {e(d)}</li>' for u, n, d in items)
    ld = {"@context": "https://schema.org", "@graph": [
        {"@type": "CollectionPage", "name": title, "url": f"{SITE}/{path}", "description": desc,
         "dateModified": TODAY, "isPartOf": {"@type": "WebSite", "name": "vanity-address", "url": f"{SITE}/"}},
        breadcrumb([("Home", f"{SITE}/"), ("All chains", f"{SITE}/{path}")])]}
    main = f'''      <p class="kicker">Directory</p>
      <h1>Vanity address generator for every chain</h1>
      <p class="lede">One offline engine, 49 blockchains. Pick a network to see its address format, valid characters, difficulty table and a ready-to-run command.</p>
      <h2>Guides</h2>
      <ul>
{lis}
      </ul>
      <h2>Also supported</h2>
      <p>The engine also covers Aptos, Algorand, Cosmos, Celestia, Osmosis, dYdX, Injective, Sei, Filecoin, Hedera, ICP, Kusama, Polkadot, MultiversX, Tezos, EVM L2s such as Base, Arbitrum and Optimism, and many Bitcoin-family forks. Browse the full list in the chain picker of the <a href="../generate/">web generator</a>.</p>
      <div class="ctas"><a class="btn btn-primary" href="../generate/">Open the generator</a></div>'''
    page = head(title, desc, path, "website") + jsonld(ld) + shell(None, main, "")
    (ROOT / "chains").mkdir(exist_ok=True)
    (ROOT / "chains" / "index.html").write_text(page)


def calc_page():
    path = "vanity-address-difficulty/"
    title = "Vanity address difficulty calculator — time estimate"
    desc = "Estimate how many attempts and how much time a vanity address pattern needs on Base58, hex, bech32 or base64 chains. Free, runs in your browser."
    faq = [("How is vanity address difficulty calculated?", "Each position in the pattern must match one specific symbol, so the expected number of attempts is the alphabet size raised to the pattern length."),
           ("Why does one extra character make it so much slower?", "Every additional character multiplies the expected work by the alphabet size: 16× for hex, 32× for bech32 or Base32, 58× for Base58, 64× for base64."),
           ("Does ignoring case make it faster?", "Yes for letters, because both upper and lower case forms match. Digits have no case variant, so patterns made of digits do not get faster."),
           ("Is the time shown exact?", "No. Finding a match is random; the average time is attempts divided by speed, and there is a 63% chance of finishing within that time and about 90% within 2.3× that time.")]
    faq_ld = {"@type": "FAQPage", "mainEntity": [
        {"@type": "Question", "name": q, "acceptedAnswer": {"@type": "Answer", "text": a}} for q, a in faq]}
    ld = {"@context": "https://schema.org", "@graph": [
        {"@type": "WebApplication", "name": "Vanity address difficulty calculator", "url": f"{SITE}/{path}",
         "applicationCategory": "UtilitiesApplication", "operatingSystem": "Any", "offers": {"@type": "Offer", "price": "0", "priceCurrency": "USD"}},
        breadcrumb([("Home", f"{SITE}/"), ("Difficulty calculator", f"{SITE}/{path}")]), faq_ld]}
    opts = "\n".join(f'          <option value="{n}">{l}</option>' for n, l in (
        (16, "Hex — EVM, Sui, NEAR (16)"), (32, "Bech32 / Base32 — Cardano, Kaspa, Stellar, SegWit (32)"),
        (58, "Base58 — Bitcoin, Solana, Tron, Dogecoin, XRP (58)"), (64, "Base64url — TON (64)")))
    tables = "\n".join(f"      <h3>{n} alphabet ({b} symbols)</h3>\n{diff_table(b)}" for n, b in (("Hex", 16), ("Base32 / bech32", 32), ("Base58", 58), ("Base64url", 64)))
    faqs = "\n".join(f"      <h3>{e(q)}</h3>\n      <p>{e(a)}</p>" for q, a in faq)
    script = '''<script>
(function () {
  var $ = function (id) { return document.getElementById(id); };
  function fmtTime(s) {
    if (s < 1) return "under a second";
    var u = [["year", 31557600], ["day", 86400], ["hour", 3600], ["minute", 60], ["second", 1]];
    for (var i = 0; i < u.length; i++) {
      if (s >= u[i][1]) { var v = s / u[i][1]; v = v < 10 ? Math.round(v * 10) / 10 : Math.round(v); return v.toLocaleString("en-US") + " " + u[i][0] + (v === 1 ? "" : "s"); }
    }
  }
  function run() {
    var a = +$("alpha").value, n = Math.max(1, Math.min(20, +$("len").value || 1)), r = Math.max(1, +$("speed").value || 1);
    var att = Math.pow(a, n);
    $("att").textContent = att.toLocaleString("en-US", { maximumFractionDigits: 0 });
    $("avg").textContent = fmtTime(att / r);
    $("p90").textContent = fmtTime(2.3026 * att / r);
  }
  ["alpha", "len", "speed"].forEach(function (id) { $(id).addEventListener("input", run); });
  run();
})();
</script>'''
    main = f'''      <p class="kicker">Tool</p>
      <h1>Vanity address difficulty calculator</h1>
      <p class="lede">Find out how many attempts and how long your custom address pattern will take, before you start grinding.</p>
      <form class="calc" onsubmit="return false">
        <label>Alphabet
          <select id="alpha">
{opts}
          </select>
        </label>
        <label>Pattern length (characters you choose)
          <input id="len" type="number" min="1" max="20" value="5" />
        </label>
        <label>Your speed (keys per second)
          <input id="speed" type="number" min="1" value="1000000" />
        </label>
      </form>
      <div class="result" aria-live="polite">
        <p>Expected attempts: <strong id="att">—</strong></p>
        <p>Average time: <strong id="avg">—</strong></p>
        <p>90% chance within: <strong id="p90">—</strong></p>
      </div>
      <p>Don't know your speed? The CLI and desktop app run a 2-second benchmark when a grind starts and then show keys per second with a live ETA. Case-insensitive matching lowers the cost for letters.</p>
      <div class="ctas"><a class="btn btn-primary" href="../generate/">Start grinding</a></div>

      <h2>Reference tables</h2>
      <p>Average time at one million keys per second. Real speed varies by chain, CPU and whether OpenCL GPU is used (EVM only).</p>
{tables}

      <h2>Frequently asked questions</h2>
{faqs}'''
    extra = '''<style>
.calc { display: grid; gap: 14px; margin: 20px 0; }
.calc label { display: grid; gap: 6px; font-weight: 600; color: var(--ink); }
.calc select, .calc input { font: inherit; padding: 10px 12px; border: 2px solid var(--ink); background: #fff; color: var(--ink); }
.result { padding: 16px 18px; border: 2px solid var(--ink); background: #fff; margin-bottom: 20px; }
.result p { margin-bottom: 6px; }
table.diff { width: 100%; border-collapse: collapse; margin: 10px 0 22px; font-size: 0.92rem; }
table.diff th, table.diff td { text-align: left; padding: 8px 10px; border-bottom: 1px solid var(--line); color: var(--ink-soft); }
table.diff th { color: var(--ink); }
</style>'''
    page = head(title, desc, path, "website", extra) + jsonld(ld) + shell(None, main, related("")) .replace("</body>", script + "\n</body>")
    (ROOT / "vanity-address-difficulty").mkdir(exist_ok=True)
    (ROOT / "vanity-address-difficulty" / "index.html").write_text(page)


def sitemap():
    pages = [("", "1.0"), ("generate/", "0.9")] + [(f"{s}/", "0.85") for s, _ in GUIDES] + [
        ("chains/", "0.8"), ("vanity-address-difficulty/", "0.8")] + [(f"{c['slug']}/", "0.7") for c in CHAINS]
    out = ['<?xml version="1.0" encoding="UTF-8"?>', '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">']
    for p, _ in pages:
        out.append(f"  <url>\n    <loc>{SITE}/{p}</loc>\n    <lastmod>{TODAY}</lastmod>\n  </url>")
    out.append("</urlset>\n")
    (ROOT / "sitemap.xml").write_text("\n".join(out))


if __name__ == "__main__":
    for c in CHAINS:
        chain_page(c)
    hub_page()
    calc_page()
    sitemap()
    print(f"generated {len(CHAINS)} chain pages + hub + calculator + sitemap")
