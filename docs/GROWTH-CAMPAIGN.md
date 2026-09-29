# Growth campaign — vanity-address v0.6.0

**Owner:** you (founder)  
**Role of this doc:** 2-week launch playbook without Twitter/X  
**Product truth:** fast, offline, multi-chain vanity address generator — keys never leave the device

---

## 1. Positioning (say this every time)

**One-liner**

> Grind a custom wallet address on your machine — 31 chains, prefix/suffix/contains, 100% offline. No servers. No telemetry.

**Why people should care**

| Pain                          | Your answer                                |
| ----------------------------- | ------------------------------------------ |
| Web vanity sites feel sketchy | Fully local CLI + desktop; open source MIT |
| Only Solana / only EVM tools  | 31 chains in one engine                    |
| Hard patterns = guesswork     | Honest ETA + contains / wildcards / OR     |
| “Is this safe?”               | Keys never leave device; verify command    |

**Primary CTA (pick one per post — don’t dump five links)**

1. Try in 10s: `npx vanity-address`
2. Site: https://vanity-address.netlify.app/
3. Source / stars: https://github.com/yudizaxay/vanity-address
4. Installers: https://github.com/yudizaxay/vanity-address/releases/latest

**Secondary (only if audience fits)**

- Rust: `cargo install vanity-address`
- Mac: `brew tap yudizaxay/tap && brew trust yudizaxay/tap && brew install vanity-address`

---

## 2. What “good result” looks like (2 weeks)

Be honest with yourself. First launch ≠ viral.

| Metric                                 | Soft win      | Strong win       |
| -------------------------------------- | ------------- | ---------------- |
| GitHub stars (net new)                 | +30–50        | +100+            |
| npm `vanity-address` installs (period) | 100–300       | 500+             |
| GitHub Release asset downloads         | 50–150        | 300+             |
| Landing unique visitors                | 500–1k        | 2k+              |
| Quality feedback (issues / comments)   | 3+ real users | 1 contributor PR |

If soft win nahi mila → posting volume / hook weak tha, product nahi.

---

## 3. Channel plan (no Twitter/X)

Do **fewer places, better posts**. Spam = dead trust for a key tool.

### Tier A — do these (highest ROI)

| Channel                                                          | Why                            | Cadence                    |
| ---------------------------------------------------------------- | ------------------------------ | -------------------------- |
| **Hacker News** — Show HN                                        | Builders + Rust curiosity      | 1 post, weekday morning US |
| **Reddit** — `r/rust`, `r/solana`, `r/ethereum`, `r/commandline` | Intentional tool seekers       | 1 post each, spaced 1 day  |
| **LinkedIn** (personal)                                          | Devs + founders share installs | 2 posts in 2 weeks         |
| **Dev.to** (or Hashnode)                                         | Evergreen SEO                  | 1 article                  |

### Tier B — optional if energy left

| Channel                                            | Notes                                                                     |
| -------------------------------------------------- | ------------------------------------------------------------------------- |
| Product Hunt                                       | Day 7–10; need 1–2 hunter friends + GIF demo                              |
| Indie Hackers                                      | “I shipped” story, not feature dump                                       |
| Relevant Discord (Solana / Ethereum _dev_ servers) | Read rules; one helpful message, no #announcements spam                   |
| YouTube Short / Reel (no X)                        | 20–30s terminal grind → match; upload YT Shorts + IG Reels if you have IG |

### Skip for now

- Bitcointalk “services” boards, airdrop Telegram, random “100x” groups
- Mass DM
- Buying fake stars / installs

---

## 4. Two-week calendar

### Before Day 1 (30–60 min)

- [ ] Landing live with Download working ([vanity-address.netlify.app](https://vanity-address.netlify.app/))
- [ ] Record **one** 20–30s silent demo: open terminal → `npx vanity-address` or short grind → match highlight
- [ ] Pin GitHub repo description to the one-liner + homepage URL
- [ ] Have accounts ready: HN, Reddit, LinkedIn, Dev.to

### Week 1 — Launch

| Day     | Action                                                                           |
| ------- | -------------------------------------------------------------------------------- |
| **1**   | LinkedIn post #1 + Dev.to article publish                                        |
| **1–2** | Show HN (US morning Tue–Thu best). Don’t cross-post HN link to Reddit same hour  |
| **2**   | Reddit `r/rust` (technical, engine + offline angle)                              |
| **3**   | Reddit `r/commandline` or `r/selfhosted` (local / privacy angle)                 |
| **4**   | Reddit `r/solana` (suffix / contains for meme coins — stay humble, safety first) |
| **5**   | Reddit `r/ethereum` or `r/ethdev` (EVM + CREATE2 lite mention)                   |
| **6–7** | Reply to every comment within 12h. Ship tiny doc fixes if people confuse install |

### Week 2 — Proof & niches

| Day       | Action                                                              |
| --------- | ------------------------------------------------------------------- |
| **8**     | LinkedIn post #2 — “what I learned shipping a local crypto tool”    |
| **9**     | Optional: Product Hunt _or_ Indie Hackers                           |
| **10**    | One Discord/dev forum reply thread (rules OK)                       |
| **11–12** | Engage: answer HN/Reddit stragglers; add FAQ to README if same Q ×3 |
| **13–14** | Measure table in §2; write 5 bullets “what worked” for next month   |

---

## 5. Copy-paste posts

Replace nothing critical — tone is calm founder, not hype.

### A) Hacker News — Show HN

**Title**

```text
Show HN: Offline multi-chain vanity address generator (31 chains, Rust)
```

**Body**

```text
I built vanity-address — a local CLI + desktop app that grinds wallet addresses
matching a prefix, suffix, or contains pattern (wildcards + OR lists).

Why local: a lot of “vanity” tools are websites. This one never phones home;
keys stay on your machine. MIT, Rust core.

- 31 chains (Solana, EVM, BTC SegWit/Taproot, Cosmos family, …)
- Honest ETA before long grinds
- npx vanity-address · cargo install vanity-address · GitHub Releases

Site: https://vanity-address.netlify.app/
Repo: https://github.com/yudizaxay/vanity-address

Happy to answer questions about the grinder, pattern engine, or chain coverage.
```

### B) Reddit — `r/rust`

**Title**

```text
vanity-address: Rust CLI/desktop for offline multi-chain vanity wallets (feedback welcome)
```

**Body**

```text
Open-source vanity address grinder with the chain logic in a `vanity-core` crate
and thin CLI / Tauri frontends.

Highlights:
- Parallel grind (rayon), system-aware worker count
- Prefix / suffix / contains + `*` wildcards + comma OR
- 31 chains behind one API
- No network during grind; MIT

Try: `cargo install vanity-address` or `npx vanity-address`
Repo: https://github.com/yudizaxay/vanity-address

Curious what Rust folks would want next (GPU path, more chains, CREATE2 depth).
```

### C) Reddit — `r/solana`

**Title**

```text
Local vanity address CLI (suffix/contains, offline) — looking for feedback
```

**Body**

```text
If you’ve used web vanity generators and felt uneasy: this runs 100% offline.

Example: grind a Solana address with a suffix / contains pattern, see ETA first,
export in Phantom-friendly form. Also supports a bunch of other chains if you
cross-ecosystem.

`npx vanity-address`
https://github.com/yudizaxay/vanity-address

Not financial advice — treat vanity keys like any wallet key. Happy to take
bug reports.
```

### D) Reddit — `r/ethereum` / `r/ethdev`

**Title**

```text
Offline vanity tool for EVM addresses (+ CREATE2 salt grind lite)
```

**Body**

```text
Built a local grinder for vanity EOAs (prefix/suffix/contains) and a CREATE2
salt grind helper for contract vanity.

Point of the project: no hosted keygen. Rust, MIT, also covers non-EVM chains
if useful.

`npx vanity-address` · https://github.com/yudizaxay/vanity-address

Feedback on the CREATE2 UX especially welcome.
```

### E) LinkedIn — Post 1 (launch)

```text
Shipped vanity-address v0.6.0 — a local multi-chain vanity wallet generator.

Problem I kept seeing: vanity tools that run in a browser and ask for trust.
This one grinds on your CPU. Keys never leave the device. Open source (MIT).

• 31 chains
• Prefix, suffix, contains (+ wildcards)
• CLI, desktop app, npm

Try: npx vanity-address
Site: https://vanity-address.netlify.app/
GitHub: https://github.com/yudizaxay/vanity-address

If you work in crypto infra or wallets, I’d love a critical eye on the security
model and docs.
```

### F) LinkedIn — Post 2 (story)

```text
Shipping a “boring” security-sensitive tool taught me more than a flashy launch.

vanity-address had to be:
1) obviously offline
2) honest about how long a pattern will take
3) boringly correct across many chains

v0.6.0 is out. If your team ever needs branded deposit addresses or you’re
evaluating vanity UX, the repo is public:

https://github.com/yudizaxay/vanity-address

Still iterating — comments welcome.
```

### G) Dev.to article outline (write once, SEO forever)

**Title:** Why your vanity address generator should never touch a server

**Sections**

1. Hook — branded wallets vs phishing anxiety
2. What a vanity address actually is (simple)
3. Threat model — why web grinders are scary
4. How vanity-address works (local grind, pattern types, 31 chains)
5. Quickstart — `npx vanity-address` + link to INSTALL
6. Closing — audit source, verify addresses, never paste keys

~800–1200 words. Embed one screenshot of CLI match or desktop result.

---

## 6. Engagement rules (this is half the campaign)

1. **Reply fast** — first 48h of each post decide the algorithm.
2. **Never argue security in bad faith** — thank, clarify, link `SECURITY.md`.
3. **Same question ×3** → add to README FAQ the same day.
4. **One CTA per post.** Extra links diluting convert.
5. **Don’t delete mild criticism** — fix or explain.

---

## 7. Demo clip recipe (20–30s)

1. Dark terminal, large font
2. `npx vanity-address` **or** short non-interactive demo if you have one
3. Show pattern like suffix `moon` on Solana (easy so clip isn’t 10 min)
4. Freeze on highlighted match + “100% local” on-screen text
5. End card: GitHub URL

Upload: YouTube Shorts + LinkedIn native video (LinkedIn often outperforms a link-only post).

---

## 8. Weekly scorecard (copy into notes)

```text
Week of: __________
HN: points ___ comments ___
Reddit posts: __ / upvotes __ / comments __
LinkedIn: reactions __ / comments __
Dev.to: views __
npm week installs: __
GH stars: __ → __
Release DLs (approx): __
Top objection heard: __________
One fix shipped from feedback: __________
```

---

## 9. After 2 weeks — decide

| If…          | Then…                                                         |
| ------------ | ------------------------------------------------------------- |
| Soft win hit | Monthly cadence: 1 deep post + reply to mentions              |
| Strong win   | Double down on best channel only; consider Product Hunt remix |
| Miss         | Rewrite hook + demo; don’t spray 10 more forums               |

Next product bets that help growth (engineering, not ads): nicer GIF in README, npm download badge, 1 “famous” chain tutorial (Solana suffix guide).

---

## 10. Quick start tomorrow morning

1. Record the 20s demo
2. Publish Dev.to from outline §5G
3. Post LinkedIn #1
4. Queue Show HN for US morning
5. Put Reddit `r/rust` on the next day — **not** same hour as HN

That’s the whole campaign. Calm, repeatable, measurable — no Twitter required.
