mod banner;
mod menu;
mod terminal;

use clap::{Parser, ValueEnum};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use menu::run as run_interactive;
use std::io::{self, Write};
use vanity_core::{grind, Chain, ChainGrinder, SystemProfile};

#[derive(Clone, ValueEnum)]
enum ChainArg {
    Sol,
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
                  and/or suffix pattern. All keys are generated locally on your machine.\n\n\
                  Run without flags for the interactive menu."
)]
struct Cli {
    #[arg(long, value_enum)]
    chain: Option<ChainArg>,

    #[arg(long)]
    prefix: Option<String>,

    #[arg(long)]
    suffix: Option<String>,

    #[arg(long)]
    exact: bool,

    #[arg(short, long)]
    quiet: bool,

    #[arg(long)]
    threads: Option<usize>,
}

struct RunConfig {
    chain: Chain,
    prefix: Option<String>,
    suffix: Option<String>,
    exact: bool,
    quiet: bool,
    threads: Option<usize>,
}

impl Cli {
    fn uses_direct_mode(&self) -> bool {
        self.chain.is_some()
            || self.prefix.is_some()
            || self.suffix.is_some()
            || self.exact
            || self.quiet
            || self.threads.is_some()
    }

    fn into_run_config(self) -> RunConfig {
        RunConfig {
            chain: self.chain.map(Into::into).unwrap_or(Chain::Solana(Default::default())),
            prefix: self.prefix,
            suffix: self.suffix,
            exact: self.exact,
            quiet: self.quiet,
            threads: self.threads,
        }
    }
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

fn print_error(msg: &str) {
    eprintln!("{} {}", "error:".red().bold(), msg);
}

fn run_grind(config: RunConfig) {
    let chain = config.chain;

    if config.exact && !chain.supports_exact_case() {
        print_error("--exact is only supported for Solana (chain: sol)");
        std::process::exit(1);
    }

    let pattern = match chain.build_pattern(
        config.prefix.as_deref(),
        config.suffix.as_deref(),
        config.exact,
    ) {
        Ok(p) => p,
        Err(e) => {
            print_error(&e);
            std::process::exit(1);
        }
    };

    let expected = chain.expected_attempts(&pattern);

    let mut profile = SystemProfile::detect();
    if let Some(threads) = config.threads {
        if threads == 0 {
            print_error("--threads must be at least 1");
            std::process::exit(1);
        }
        profile = profile.with_threads(threads);
    }

    if !config.quiet {
        println!();
        println!(
            "  {}  {}",
            "System".dimmed(),
            profile.summary_line().cyan()
        );
        println!(
            "  {}  {}",
            "CPU".dimmed(),
            profile.cpu_description()
        );
        println!(
            "  {}  {}",
            "Workers".dimmed(),
            profile.worker_description().green()
        );
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

    let pb = if config.quiet {
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

    let result = match grind(
        chain.clone(),
        pattern,
        &profile,
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
    ) {
        Ok(r) => r,
        Err(e) => {
            print_error(&e);
            std::process::exit(1);
        }
    };

    if let Some(ref bar) = pb {
        bar.finish_and_clear();
    }

    if !config.quiet {
        println!();
        println!("{}", " Match found! ".black().on_green().bold());
        println!();
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
        if config.quiet {
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

    if !config.quiet {
        println!();
        println!(
            "  {}",
            "Verify the address before sending funds.".yellow().dimmed()
        );
    }

    let _ = io::stdout().flush();
}

fn main() {
    terminal::install_ctrlc_handler();

    let cli = Cli::parse();

    if cli.uses_direct_mode() {
        run_grind(cli.into_run_config());
        return;
    }

    // Interactive old-school menu mode (default)
    loop {
        let Some(config) = run_interactive() else {
            return;
        };

        run_grind(RunConfig {
            chain: config.chain,
            prefix: config.prefix,
            suffix: config.suffix,
            exact: config.exact,
            quiet: false,
            threads: None,
        });

        println!();
        let again = terminal::read_yes_no_key("  Grind another? [y/N]: ", false).unwrap_or(false);

        if !again {
            terminal::peace_out();
            break;
        }
    }
}
