use clap::{Parser, ValueEnum};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{self, Write};
use vanity_core::{grind, Chain, ChainGrinder};

#[derive(Clone, ValueEnum)]
enum ChainArg {
    /// Solana (base58 addresses)
    Sol,
    /// EVM / Ethereum (0x hex addresses)
    Evm,
}

impl From<ChainArg> for Chain {
    fn from(arg: ChainArg) -> Self {
        match arg {
            ChainArg::Sol => Chain::Solana(Default::default()),
            ChainArg::Evm => Chain::Evm(Default::default()),
        }
    }
}

#[derive(Parser)]
#[command(
    name = "vanity-address",
    version,
    author,
    about = "Fast, local multi-chain vanity address generator",
    long_about = "Generate cryptocurrency keypairs whose public address matches a prefix \
                  and/or suffix pattern. All keys are generated locally on your machine — \
                  nothing is ever sent over the network."
)]
struct Cli {
    /// Blockchain to grind addresses for
    #[arg(long, value_enum, default_value = "sol")]
    chain: ChainArg,

    /// Address must start with this pattern
    #[arg(long)]
    prefix: Option<String>,

    /// Address must end with this pattern
    #[arg(long)]
    suffix: Option<String>,

    /// Require exact casing (Solana only — EVM is always lowercase hex)
    #[arg(long)]
    exact: bool,

    /// Minimal output — only print keys on success
    #[arg(short, long)]
    quiet: bool,
}

fn format_attempts(n: f64) -> String {
    if n >= 1_000_000_000.0 {
        format!("{:.1}B", n / 1_000_000_000.0)
    } else if n >= 1_000_000.0 {
        format!("{:.1}M", n / 1_000_000.0)
    } else if n >= 1_000.0 {
        format!("{:.1}K", n / 1_000.0)
    } else {
        format!("{:.0}", n)
    }
}

fn print_banner(quiet: bool) {
    if quiet {
        return;
    }
    println!(
        "{}",
        "vanity-address".bold().cyan()
    );
    println!(
        "{}",
        "Local multi-chain vanity address generator".dimmed()
    );
    println!();
}

fn print_error(msg: &str) {
    eprintln!("{} {}", "error:".red().bold(), msg);
}

fn print_success_header(quiet: bool) {
    if quiet {
        return;
    }
    println!();
    println!("{}", " Match found! ".black().on_green().bold());
    println!();
}

fn main() {
    let cli = Cli::parse();
    let chain: Chain = cli.chain.into();

    if cli.exact && !chain.supports_exact_case() {
        print_error("--exact is only supported for Solana (chain: sol)");
        std::process::exit(1);
    }

    let pattern = match chain.build_pattern(
        cli.prefix.as_deref(),
        cli.suffix.as_deref(),
        cli.exact,
    ) {
        Ok(p) => p,
        Err(e) => {
            print_error(&e);
            std::process::exit(1);
        }
    };

    let expected = chain.expected_attempts(&pattern);

    print_banner(cli.quiet);

    if !cli.quiet {
        println!(
            "  {}  {}",
            "Chain".dimmed(),
            chain.display_name().bold()
        );
        println!(
            "  {}  {}",
            "Target".dimmed(),
            pattern.description().bold()
        );
        println!(
            "  {}  {}",
            "Mode".dimmed(),
            pattern.case_mode()
        );
        println!(
            "  {}  ~{} attempts (average)",
            "Expected".dimmed(),
            format_attempts(expected).yellow()
        );
        println!(
            "  {}  {}",
            "Hint".dimmed(),
            chain.pattern_hint().dimmed()
        );
        println!();
        println!("{}", "Grinding...  (Ctrl+C to stop)".dimmed());
        println!();
    }

    let pb = if cli.quiet {
        None
    } else {
        let bar = ProgressBar::new_spinner();
        bar.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.cyan} {msg}")
                .unwrap(),
        );
        bar.enable_steady_tick(std::time::Duration::from_millis(80));
        Some(bar)
    };

    let result = grind(
        chain.clone(),
        pattern,
        250_000,
        |attempts, rate, eta_min| {
            if let Some(ref bar) = pb {
                bar.set_message(format!(
                    "{} keys | {} keys/s | ~{:.0} min remaining",
                    format_attempts(attempts as f64).cyan(),
                    format!("{rate:.0}").green(),
                    eta_min
                ));
            }
        },
    );

    if let Some(ref bar) = pb {
        bar.finish_and_clear();
    }

    let Some(result) = result else {
        print_error("grinding stopped before a match was found");
        std::process::exit(1);
    };

    print_success_header(cli.quiet);

    if !cli.quiet {
        println!(
            "  {}  {}",
            "Address".green().bold(),
            result.keypair.address.bold().white()
        );
        println!(
            "  {}  {:.2}s",
            "Time".dimmed(),
            result.elapsed_secs
        );
        println!(
            "  {}  {} ({:.0} keys/s)",
            "Attempts".dimmed(),
            result.attempts,
            result.attempts as f64 / result.elapsed_secs
        );
        println!();
        println!("{}", " Private Keys ".black().on_red().bold());
        println!(
            "  {}",
            "Never share these with anyone.".red().dimmed()
        );
        println!();
    } else {
        println!("{}", result.keypair.address);
    }

    for export in &result.keypair.exports {
        if cli.quiet {
            println!("{}: {}", export.label, export.value);
        } else {
            println!(
                "  {}  {}",
                export.label.dimmed(),
                export.value.bold()
            );
            if let Some(ref hint) = export.hint {
                println!("         {}", hint.dimmed());
            }
        }
    }

    if !cli.quiet {
        println!();
        println!(
            "  {}",
            "Verify the address before sending funds.".yellow().dimmed()
        );
    }

    let _ = io::stdout().flush();
}
