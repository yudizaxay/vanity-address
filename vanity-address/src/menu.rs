use crate::banner;
use crate::terminal::{peace_out, read_line_with_escape, read_menu_choice, read_yes_no_key, wait_for_key, MenuChoice};
use colored::Colorize;
use std::io::Write;
use vanity_core::{Chain, ChainGrinder, SystemProfile};

#[derive(Clone, Copy, PartialEq, Eq)]
enum WizardStep {
    Chain,
    MatchKind,
    Pattern,
    CaseMode,
    Summary,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MatchKind {
    Suffix,
    Prefix,
    Both,
}

pub struct InteractiveConfig {
    pub chain: Chain,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub exact: bool,
}

pub fn run() -> Option<InteractiveConfig> {
    loop {
        match show_main_menu() {
            MainAction::Start => {
                if let Some(config) = run_wizard() {
                    return Some(config);
                }
            }
            MainAction::Help => show_help(),
            MainAction::Exit => {
                clear_screen();
                peace_out();
                return None;
            }
        }
    }
}

enum MainAction {
    Start,
    Help,
    Exit,
}

fn show_main_menu() -> MainAction {
    clear_screen();
    banner::print_splash();
    println!("  {}  Start a new grind", "[1]".green().bold());
    println!("  {}  Help & pattern rules", "[2]".yellow().bold());
    println!("  {}  Exit", "[3]".red().bold());
    println!();

    match read_menu_choice("  Press [1-3]: ", 1, 3, false) {
        MenuChoice::Selected(1) => MainAction::Start,
        MenuChoice::Selected(2) => MainAction::Help,
        MenuChoice::Selected(3) => MainAction::Exit,
        MenuChoice::Back | MenuChoice::Selected(_) => MainAction::Exit,
    }
}

fn run_wizard() -> Option<InteractiveConfig> {
    let mut step = WizardStep::Chain;
    let mut chain = Chain::Solana(Default::default());
    let mut match_kind = MatchKind::Suffix;
    let mut prefix: Option<String> = None;
    let mut suffix: Option<String> = None;
    let mut exact = false;

    loop {
        clear_screen();

        match step {
            WizardStep::Chain => {
                banner::print_compact();
                println!("{}", "── Step 1/4 · Select blockchain ──".bold().cyan());
                println!();
                println!("  {}  Solana  (base58 · Phantom, Solflare)", "[1]".green());
                println!("  {}  EVM     (0x hex · MetaMask)", "[2]".green());
                println!("  {}  Back to main menu", "[0]".dimmed());
                println!();

                match read_menu_choice("  Press [0-2]: ", 1, 2, true) {
                    MenuChoice::Back => return None,
                    MenuChoice::Selected(1) => {
                        chain = Chain::Solana(Default::default());
                        step = WizardStep::MatchKind;
                    }
                    MenuChoice::Selected(2) => {
                        chain = Chain::Evm(Default::default());
                        step = WizardStep::MatchKind;
                    }
                    _ => {}
                }
            }

            WizardStep::MatchKind => {
                banner::print_compact();
                println!("{}", "── Step 2/4 · Prefix or suffix? ──".bold().cyan());
                println!();
                println!("  {}  Suffix  — address ends with...", "[1]".green());
                println!("  {}  Prefix  — address starts with...", "[2]".green());
                println!("  {}  Both    — prefix + suffix", "[3]".green());
                println!("  {}  Back", "[0]".dimmed());
                println!();

                match read_menu_choice("  Press [0-3]: ", 1, 3, true) {
                    MenuChoice::Back => step = WizardStep::Chain,
                    MenuChoice::Selected(1) => {
                        match_kind = MatchKind::Suffix;
                        step = WizardStep::Pattern;
                    }
                    MenuChoice::Selected(2) => {
                        match_kind = MatchKind::Prefix;
                        step = WizardStep::Pattern;
                    }
                    MenuChoice::Selected(3) => {
                        match_kind = MatchKind::Both;
                        step = WizardStep::Pattern;
                    }
                    _ => {}
                }
            }

            WizardStep::Pattern => {
                banner::print_compact();
                let label = match match_kind {
                    MatchKind::Suffix => "Enter suffix text",
                    MatchKind::Prefix => "Enter prefix text",
                    MatchKind::Both => "Enter prefix & suffix",
                };

                println!("{}", format!("── Step 3/4 · {label} ──").bold().cyan());
                println!();
                println!("  {}", chain.pattern_hint().dimmed());
                println!("  {}", "[Esc] Back".dimmed());
                println!();

                let back_step = WizardStep::MatchKind;
                let input_ok = match match_kind {
                    MatchKind::Suffix => match read_line_with_escape("  Suffix: ") {
                        None => {
                            step = back_step;
                            continue;
                        }
                        Some(text) if text.is_empty() => {
                            show_error("Suffix cannot be empty.");
                            continue;
                        }
                        Some(text) => {
                            prefix = None;
                            suffix = Some(text);
                            true
                        }
                    },
                    MatchKind::Prefix => match read_line_with_escape("  Prefix: ") {
                        None => {
                            step = back_step;
                            continue;
                        }
                        Some(text) if text.is_empty() => {
                            show_error("Prefix cannot be empty.");
                            continue;
                        }
                        Some(text) => {
                            prefix = Some(text);
                            suffix = None;
                            true
                        }
                    },
                    MatchKind::Both => {
                        let pre = match read_line_with_escape("  Prefix: ") {
                            None => {
                                step = back_step;
                                continue;
                            }
                            Some(v) => v,
                        };
                        let suf = match read_line_with_escape("  Suffix: ") {
                            None => {
                                step = WizardStep::Pattern;
                                continue;
                            }
                            Some(v) => v,
                        };
                        if pre.is_empty() || suf.is_empty() {
                            show_error("Both prefix and suffix are required.");
                            continue;
                        }
                        prefix = Some(pre);
                        suffix = Some(suf);
                        true
                    }
                };

                if input_ok {
                    if chain.supports_exact_case() {
                        step = WizardStep::CaseMode;
                    } else {
                        exact = false;
                        step = WizardStep::Summary;
                    }
                }
            }

            WizardStep::CaseMode => {
                banner::print_compact();
                println!("{}", "── Step 4/4 · Case sensitivity ──".bold().cyan());
                println!();
                println!("  {}  Any case  (faster — recommended)", "[1]".green());
                println!("  {}  Exact case (slower)", "[2]".yellow());
                println!("  {}  Back", "[0]".dimmed());
                println!();

                match read_menu_choice("  Press [0-2]: ", 1, 2, true) {
                    MenuChoice::Back => step = WizardStep::Pattern,
                    MenuChoice::Selected(1) => {
                        exact = false;
                        step = WizardStep::Summary;
                    }
                    MenuChoice::Selected(2) => {
                        exact = true;
                        step = WizardStep::Summary;
                    }
                    _ => {}
                }
            }

            WizardStep::Summary => {
                let pattern = match chain.build_pattern(
                    prefix.as_deref(),
                    suffix.as_deref(),
                    exact,
                ) {
                    Ok(p) => p,
                    Err(e) => {
                        show_error(&e);
                        step = WizardStep::Pattern;
                        continue;
                    }
                };

                let profile = SystemProfile::detect();
                let expected = chain.expected_attempts(&pattern);
                let estimate = TimeEstimate::from_attempts(expected, &profile, chain.id());

                banner::print_compact();
                print_summary(&chain, &pattern, &estimate, &profile);
                println!("  {}  Start grinding", "[y]".green().bold());
                println!("  {}  Back to edit", "[Esc]".dimmed());
                println!();

                match read_yes_no_key("  Press [y/N]: ", true) {
                    Some(true) => {
                        return Some(InteractiveConfig {
                            chain,
                            prefix,
                            suffix,
                            exact,
                        });
                    }
                    Some(false) => return None,
                    None => {
                        step = if chain.supports_exact_case() {
                            WizardStep::CaseMode
                        } else {
                            WizardStep::Pattern
                        };
                    }
                }
            }
        }
    }
}

fn show_help() {
    clear_screen();
    banner::print_splash();
    println!("{}", "── Help & pattern rules ──".bold().cyan());
    println!();
    println!("  {}", "What is vanity-address?".bold());
    println!("  Generates wallet addresses matching a custom");
    println!("  prefix or suffix — e.g. ending in 'axay'.");
    println!();
    println!("  {}", "Chains".bold());
    println!("  • Solana  — base58 (no 0, O, I, l)");
    println!("  • EVM     — hex (0-9, a-f)");
    println!();
    println!("  {}", "Security".bold().red());
    println!("  • 100% local · no internet");
    println!("  • NEVER share your private key");
    println!();
    println!("  {}", "Navigation".bold());
    println!("  • Press number keys — no Enter needed");
    println!("  • [0] or Esc — go back");
    println!();
    pause();
}

fn show_error(msg: &str) {
    clear_screen();
    banner::print_compact();
    println!("  {} {}", "✗".red(), msg.red());
    pause();
}

struct TimeEstimate {
    attempts_label: String,
    time_label: String,
    difficulty: String,
    difficulty_bars: String,
}

impl TimeEstimate {
    fn from_attempts(attempts: f64, profile: &SystemProfile, chain_id: &str) -> Self {
        let per_thread = if chain_id == "evm" { 35_000.0 } else { 80_000.0 };
        let keys_per_sec = per_thread * profile.worker_threads as f64;
        let avg_secs = attempts / keys_per_sec;
        let attempts_label = format_attempts(attempts);

        let time_label = if avg_secs < 5.0 {
            "a few seconds".to_string()
        } else if avg_secs < 60.0 {
            format!("~{:.0} seconds", avg_secs)
        } else if avg_secs < 3_600.0 {
            format!("~{:.0} minutes", avg_secs / 60.0)
        } else if avg_secs < 86_400.0 {
            format!("~{:.1} hours", avg_secs / 3_600.0)
        } else if avg_secs < 86_400.0 * 30.0 {
            format!("~{:.0} days", avg_secs / 86_400.0)
        } else {
            "weeks to months (very long!)".to_string()
        };

        let (difficulty, filled) = match attempts {
            a if a < 100_000.0 => ("Easy", 2),
            a if a < 10_000_000.0 => ("Quick", 3),
            a if a < 1_000_000_000.0 => ("Medium", 5),
            a if a < 1_000_000_000_000.0 => ("Hard", 7),
            _ => ("Extreme", 10),
        };

        Self {
            attempts_label,
            time_label,
            difficulty: difficulty.to_string(),
            difficulty_bars: format!("{}{}", "█".repeat(filled), "░".repeat(10 - filled)),
        }
    }
}

fn print_summary(
    chain: &Chain,
    pattern: &vanity_core::Pattern,
    estimate: &TimeEstimate,
    profile: &SystemProfile,
) {
    println!("{}", "── Summary ──".bold().yellow());
    println!();
    println!("  {:<14} {}", "Chain:".dimmed(), chain.display_name().bold());
    println!("  {:<14} {}", "Target:".dimmed(), pattern.description().bold());
    println!("  {:<14} {}", "Mode:".dimmed(), pattern.case_mode());
    println!(
        "  {:<14} {}",
        "Attempts:".dimmed(),
        format!("~{} (avg)", estimate.attempts_label).yellow()
    );
    println!(
        "  {:<14} {}",
        "Est. time:".dimmed(),
        estimate.time_label.green()
    );
    println!(
        "  {:<14} {} {}",
        "Difficulty:".dimmed(),
        estimate.difficulty_bars,
        estimate.difficulty.bold()
    );

    println!();
    println!("{}", "── Your System ──".bold().cyan());
    println!();
    println!(
        "  {:<14} {}",
        "CPU:".dimmed(),
        profile.cpu_description().cyan()
    );
    println!(
        "  {:<14} {}",
        "Workers:".dimmed(),
        profile.worker_description().green()
    );
    println!(
        "  {:<14} {:.1} GB total · {:.1} GB available",
        "RAM:".dimmed(),
        profile.total_memory_mb as f64 / 1024.0,
        profile.available_memory_mb as f64 / 1024.0,
    );
    println!(
        "  {:<14} {}",
        "Memory load:".dimmed(),
        format!("{}", profile.memory_pressure).cyan()
    );
    let speed = profile.estimated_keys_per_sec(chain.id());
    println!(
        "  {:<14} {}",
        "Est. speed:".dimmed(),
        format!("~{} keys/sec on this machine", format_speed(speed)).green()
    );

    if pattern.has_suffix() && !pattern.has_prefix() {
        println!();
        let len = pattern.suffix.len();
        print_length_guide(chain.id(), len);
    }
    println!();
}

fn format_speed(n: f64) -> String {
    if n >= 1_000_000.0 {
        format!("{:.1}M", n / 1_000_000.0)
    } else if n >= 1_000.0 {
        format!("{:.0}K", n / 1_000.0)
    } else {
        format!("{:.0}", n)
    }
}

fn print_length_guide(chain_id: &str, len: usize) {
    println!("  {}", format!("{len}-char pattern guide:").dimmed());
    if chain_id == "evm" {
        println!("    2 chars → seconds · 4 → ~1 min · 6 → ~30 min · 8+ → hours");
    } else {
        println!("    2 chars → seconds · 4 → ~1 min · 6 → ~1 hour · 8+ → days");
    }
}

fn pause() {
    wait_for_key("\n  Press any key to continue...");
}

fn clear_screen() {
    print!("\x1B[2J\x1B[H");
    let _ = std::io::stdout().flush();
}

fn format_attempts(n: f64) -> String {
    if n >= 1_000_000_000_000.0 {
        format!("{:.1}T", n / 1_000_000_000_000.0)
    } else if n >= 1_000_000_000.0 {
        format!("{:.1}B", n / 1_000_000_000.0)
    } else if n >= 1_000_000.0 {
        format!("{:.1}M", n / 1_000_000.0)
    } else if n >= 1_000.0 {
        format!("{:.1}K", n / 1_000.0)
    } else {
        format!("{:.0}", n)
    }
}
