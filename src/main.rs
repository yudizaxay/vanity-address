use clap::Parser;
use rayon::prelude::*;
use solana_sdk::signature::{Keypair, Signer};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

const BASE58_INVALID: &str = "0OIl";

#[derive(Parser)]
#[command(
    name = "vanity-orbt",
    about = "Fast, local Solana vanity address generator",
    long_about = "Generate Solana keypairs whose public address matches a prefix and/or suffix pattern.\n\
                  All keys are generated locally — nothing is sent over the network."
)]
struct Cli {
    /// Address must start with this pattern (base58 characters only)
    #[arg(long)]
    prefix: Option<String>,

    /// Address must end with this pattern (base58 characters only)
    #[arg(long)]
    suffix: Option<String>,

    /// Require exact casing instead of case-insensitive matching
    #[arg(long)]
    exact: bool,
}

fn validate_pattern(label: &str, pattern: &str) {
    for c in pattern.chars() {
        if BASE58_INVALID.contains(c) || !c.is_ascii_alphanumeric() {
            eprintln!(
                "'{label}' contains '{c}', which never appears in a base58 address"
            );
            std::process::exit(1);
        }
    }
}

fn char_combinations(pattern: &str, ignore_case: bool) -> f64 {
    pattern
        .chars()
        .map(|c| {
            if ignore_case && c.is_ascii_alphabetic() {
                29.0
            } else {
                58.0
            }
        })
        .product()
}

fn matches_pattern(addr: &str, pattern: &str, ignore_case: bool, at_start: bool) -> bool {
    if pattern.is_empty() {
        return true;
    }

    let addr_bytes = addr.as_bytes();
    let pat_bytes = pattern.as_bytes();

    let slice = if at_start {
        &addr_bytes[..pat_bytes.len()]
    } else {
        &addr_bytes[addr_bytes.len() - pat_bytes.len()..]
    };

    if ignore_case {
        slice.eq_ignore_ascii_case(pat_bytes)
    } else {
        slice == pat_bytes
    }
}

fn main() {
    let cli = Cli::parse();

    let prefix = cli.prefix.unwrap_or_default();
    let suffix = cli.suffix.unwrap_or_else(|| "orbt".to_string());

    if prefix.is_empty() && suffix.is_empty() {
        eprintln!("Provide at least one of --prefix or --suffix");
        std::process::exit(1);
    }

    if !prefix.is_empty() {
        validate_pattern("prefix", &prefix);
    }
    if !suffix.is_empty() {
        validate_pattern("suffix", &suffix);
    }

    let ignore_case = !cli.exact;
    let prefix_pat = if ignore_case {
        prefix.to_ascii_lowercase()
    } else {
        prefix.clone()
    };
    let suffix_pat = if ignore_case {
        suffix.to_ascii_lowercase()
    } else {
        suffix.clone()
    };

    let mut combos = 1.0_f64;
    if !prefix.is_empty() {
        combos *= char_combinations(&prefix, ignore_case);
    }
    if !suffix.is_empty() {
        combos *= char_combinations(&suffix, ignore_case);
    }

    let match_desc = match (&prefix.is_empty(), &suffix.is_empty()) {
        (false, false) => format!(
            "starting with '{}' and ending with '{}'",
            prefix, suffix
        ),
        (false, true) => format!("starting with '{}'", prefix),
        (true, false) => format!("ending with '{}'", suffix),
        (true, true) => unreachable!(),
    };

    println!(
        "Grinding for a Solana address {match_desc} ({})",
        if ignore_case {
            "any case"
        } else {
            "exact case"
        }
    );
    println!(
        "Expected attempts: ~{:.1} million (average — could be more or less)",
        combos / 1e6
    );

    let counter = AtomicU64::new(0);
    let start = Instant::now();

    let result = rayon::iter::repeat(())
        .map(|_| {
            let n = counter.fetch_add(1, Ordering::Relaxed) + 1;
            if n % 1_000_000 == 0 {
                let secs = start.elapsed().as_secs_f64();
                let rate = n as f64 / secs;
                eprintln!(
                    "  {:>5.1}M keys | {:>7.0} keys/s | ~{:.0} min to expected match",
                    n as f64 / 1e6,
                    rate,
                    (combos - n as f64).max(0.0) / rate / 60.0
                );
            }
            Keypair::new()
        })
        .find_any(|keypair| {
            let addr = keypair.pubkey().to_string();
            let prefix_ok = prefix.is_empty()
                || matches_pattern(&addr, &prefix_pat, ignore_case, true);
            let suffix_ok = suffix.is_empty()
                || matches_pattern(&addr, &suffix_pat, ignore_case, false);
            prefix_ok && suffix_ok
        });

    if let Some(keypair) = result {
        let elapsed = start.elapsed();
        let attempts = counter.load(Ordering::Relaxed);

        println!("\nSUCCESS!");
        println!("Public Key: {}", keypair.pubkey());
        println!("Time: {:.2} seconds", elapsed.as_secs_f64());
        println!("Attempts: {}", attempts);
        println!(
            "Speed: {:.0} keys/sec",
            attempts as f64 / elapsed.as_secs_f64()
        );

        println!(
            "Private Key (hex): {}",
            hex::encode(keypair.secret().to_bytes())
        );
        println!(
            "Private Key (base58, wallet import): {}",
            bs58::encode(keypair.to_bytes()).into_string()
        );
        println!(
            "Keypair (solana-cli JSON): {:?}",
            keypair.to_bytes().to_vec()
        );
    }
}
