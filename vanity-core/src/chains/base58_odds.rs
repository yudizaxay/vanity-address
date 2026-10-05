//! Exact prefix odds for base58 addresses.
//!
//! A base58 address encodes `version` bytes followed by uniformly random bytes
//! (hash or public key + checksum). The version pins the numeric range, so the
//! leading characters are limited to a range (Doge: `D` then `5`–`U`) and are
//! not equally likely. Counting "58 options per character" both accepts
//! impossible prefixes and misjudges the attempts needed.

use crate::pattern::Pattern;

use super::util::{apply_address_start, base58_combinations, expected_from_pattern};

/// Overlaps smaller than this fraction of a digit slot are float noise or so rare
/// they would need ~10⁹× the normal attempts — treat them as impossible.
const NEGLIGIBLE: f64 = 1e-9;

#[derive(Clone, Copy)]
pub struct Base58Layout<'a> {
    pub version: &'a [u8],
    /// Random bytes after the version (hash/pubkey + checksum).
    pub random_len: usize,
    pub alphabet: &'a str,
}

impl Base58Layout<'_> {
    /// Chance that a random address starts with `prefix`.
    pub fn prefix_probability(&self, prefix: &str, ignore_case: bool) -> f64 {
        self.probability(&self.digit_sets(prefix, ignore_case))
    }

    /// Rejects prefixes no address of this layout can start with, naming the
    /// characters that can appear at the first impossible position.
    pub fn check_prefix(&self, chain_name: &str, pattern: &Pattern) -> Result<(), String> {
        if !pattern.has_prefix() {
            return Ok(());
        }
        let sets = self.digit_sets(&pattern.prefix, pattern.ignore_case);
        if self.probability(&sets) > 0.0 {
            return Ok(());
        }
        let bad = (1..=sets.len())
            .find(|&n| self.probability(&sets[..n]) == 0.0)
            .unwrap_or(sets.len())
            - 1;
        let before: String = pattern.prefix.chars().take(bad).collect();
        let c = pattern.prefix.chars().nth(bad).unwrap_or('?');
        let mut probe = sets[..bad].to_vec();
        probe.push(Vec::new());
        let options: String = self
            .alphabet
            .chars()
            .enumerate()
            .filter(|&(d, _)| {
                probe[bad] = vec![d as u32];
                self.probability(&probe) > 0.0
            })
            .map(|(_, ch)| ch)
            .collect();
        Err(if before.is_empty() {
            format!("{chain_name} addresses can never start with '{c}'. The first character is always one of: {options}")
        } else {
            format!("{chain_name} addresses can never have '{c}' after '{before}'. The next character is always one of: {options}")
        })
    }

    /// Adds the always-present `lead` and validates the prefix. A prefix that already
    /// starts with the lead is ambiguous (Doge `D1` = `D1…` or `DD1…`), so the
    /// reading that can actually occur wins.
    pub fn apply_lead(
        &self,
        chain_name: &str,
        pattern: &mut Pattern,
        lead: &str,
    ) -> Result<(), String> {
        let typed = pattern.clone();
        apply_address_start(pattern, chain_name, lead, &[])?;
        let as_typed = self.check_prefix(chain_name, pattern);
        if as_typed.is_err() && pattern.prefix.len() == typed.prefix.len() {
            let mut alt = typed;
            alt.prefix = format!("{lead}{}", alt.prefix);
            apply_address_start(&mut alt, chain_name, lead, &[])?;
            if self.check_prefix(chain_name, &alt).is_ok() {
                *pattern = alt;
                return Ok(());
            }
        }
        as_typed
    }

    /// Expected attempts: exact odds for the prefix, the usual estimate for suffix/contains.
    pub fn expected_attempts(&self, pattern: &Pattern) -> f64 {
        let rest = expected_from_pattern(pattern, base58_combinations)
            / base58_combinations(pattern.user_prefix());
        if !pattern.has_prefix() {
            return rest;
        }
        rest / self.prefix_probability(&pattern.prefix, pattern.ignore_case)
    }

    fn digit_sets(&self, prefix: &str, ignore_case: bool) -> Vec<Vec<u32>> {
        let digit = |c: char| self.alphabet.find(c).map(|i| i as u32);
        prefix
            .chars()
            .map(|c| {
                if c == '*' {
                    return (0..self.alphabet.len() as u32).collect();
                }
                let mut set: Vec<u32> = digit(c).into_iter().collect();
                if ignore_case {
                    let other = if c.is_ascii_lowercase() {
                        c.to_ascii_uppercase()
                    } else {
                        c.to_ascii_lowercase()
                    };
                    if other != c {
                        set.extend(digit(other));
                    }
                }
                set
            })
            .collect()
    }

    fn probability(&self, sets: &[Vec<u32>]) -> f64 {
        let all_ones = |s: &[Vec<u32>]| s.iter().all(|set| set.contains(&0));
        let zeros = self.version.iter().take_while(|&&b| b == 0).count();
        let head = zeros.min(sets.len());
        if !all_ones(&sets[..head]) {
            return 0.0;
        }
        let sets = &sets[head..];
        let random = self.random_len as i32;
        let rest = &self.version[zeros..];

        if !rest.is_empty() {
            let v = rest.iter().fold(0.0, |acc, &b| acc * 256.0 + b as f64);
            let scale = 256f64.powi(random);
            return digits_probability(v * scale, (v + 1.0) * scale, sets);
        }

        // All-zero version: each leading zero byte of the random part adds another `1`.
        let mut total = 0.0;
        let mut at_least_k = 1.0;
        for k in 0..self.random_len {
            if !all_ones(&sets[..k]) {
                break;
            }
            if k == sets.len() {
                return total + at_least_k;
            }
            let lo = 256f64.powi(random - k as i32 - 1);
            total += at_least_k * (255.0 / 256.0) * digits_probability(lo, lo * 256.0, &sets[k..]);
            at_least_k /= 256.0;
        }
        total
    }
}

/// Chance that the base58 digits of a number uniform in `[lo, hi)` start with
/// one digit from each of `sets`.
fn digits_probability(lo: f64, hi: f64, sets: &[Vec<u32>]) -> f64 {
    if sets.is_empty() {
        return 1.0;
    }
    let mut total = 0.0;
    for len in 1.. {
        let len_lo = if len == 1 { 0.0 } else { 58f64.powi(len - 1) };
        if len_lo >= hi {
            break;
        }
        let (a, b) = (lo.max(len_lo), hi.min(58f64.powi(len)));
        if a < b && sets.len() <= len as usize {
            total += measure(0.0, 58f64.powi(len - 1), sets, a, b);
        }
    }
    total / (hi - lo)
}

/// Size of `[a, b)` covered by numbers in `[start, start + 58·slot)` whose next
/// digits come from `sets`.
fn measure(start: f64, slot: f64, sets: &[Vec<u32>], a: f64, b: f64) -> f64 {
    let mut sum = 0.0;
    for &d in &sets[0] {
        let s = start + d as f64 * slot;
        let e = s + slot;
        let overlap = e.min(b) - s.max(a);
        if overlap <= slot * NEGLIGIBLE {
            continue;
        }
        sum += if sets.len() == 1 {
            overlap
        } else if s >= a && e <= b {
            slot * sets[1..]
                .iter()
                .map(|set| set.len() as f64 / 58.0)
                .product::<f64>()
        } else {
            measure(s, slot / 58.0, &sets[1..], a, b)
        };
    }
    sum
}
