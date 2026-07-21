use crate::banner;
use crate::terminal::{
    peace_out, read_line_with_escape, read_menu_choice, read_yes_no_key, wait_for_key, MenuChoice,
};
use crate::warnings;
use colored::Colorize;
use vanity_core::{
    grind_estimate, Chain, ChainGrinder, GrindEstimate, PatternRisk, SystemProfile, MENU_CHAINS,
};

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
                for (i, (_, label)) in MENU_CHAINS.iter().enumerate() {
                    println!("  {}  {}", format!("[{}]", i + 1).green(), label);
                }
                println!("  {}  Back to main menu", "[0]".dimmed());
                if MENU_CHAINS.len() > 9 {
                    println!();
                    println!(
                        "  {}",
                        "Tip: [10–13] type both digits within 2s · [1] then Enter for Solana"
                            .dimmed()
                    );
                }
                println!();

                let max = MENU_CHAINS.len() as u32;
                let prompt = format!("  Press [0-{max}]: ");
                match read_menu_choice(&prompt, 1, max, true) {
                    MenuChoice::Back => return None,
                    MenuChoice::Selected(n) => {
                        if let Some(selected) = Chain::from_menu_index((n - 1) as usize) {
                            chain = selected;
                            step = WizardStep::MatchKind;
                        }
                    }
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
                let pattern = match chain.build_pattern(prefix.as_deref(), suffix.as_deref(), exact)
                {
                    Ok(p) => p,
                    Err(e) => {
                        show_error(&e);
                        step = WizardStep::Pattern;
                        continue;
                    }
                };

                let profile = SystemProfile::detect();
                let expected = chain.expected_attempts(&pattern);
                let estimate = grind_estimate(
                    expected,
                    profile.estimated_keys_per_sec(chain.id()),
                    &pattern,
                );

                banner::print_compact();
                print_summary(&chain, &pattern, &estimate, &profile);

                if estimate.risk == PatternRisk::Impractical {
                    println!();
                    println!(
                        "  {}  {}",
                        "[y] / Enter".yellow().bold(),
                        "Start anyway (not recommended)".yellow()
                    );
                } else {
                    println!("  {}  Start grinding", "[y] / Enter".green().bold());
                }
                println!("  {}  Back to edit", "[Esc]".dimmed());
                println!();

                let start = match read_yes_no_key("  Press [y/Enter]: ", true, true) {
                    Some(true) if estimate.risk == PatternRisk::Impractical => {
                        println!();
                        println!(
                            "  {}",
                            "⛔ Last chance — this grind may never finish.".red().bold()
                        );
                        read_yes_no_key("  Really start? [y/N]: ", false, false) == Some(true)
                    }
                    Some(true) => true,
                    _ => false,
                };

                if start {
                    return Some(InteractiveConfig {
                        chain,
                        prefix,
                        suffix,
                        exact,
                    });
                }

                step = if chain.supports_exact_case() {
                    WizardStep::CaseMode
                } else {
                    WizardStep::Pattern
                };
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
    for (id, label) in MENU_CHAINS {
        println!("  • {id} — {label}");
    }
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

fn print_summary(
    chain: &Chain,
    pattern: &vanity_core::Pattern,
    estimate: &GrindEstimate,
    profile: &SystemProfile,
) {
    println!("{}", "── Summary ──".bold().yellow());
    println!();
    println!(
        "  {:<14} {}",
        "Chain:".dimmed(),
        chain.display_name().bold()
    );
    println!(
        "  {:<14} {}",
        "Target:".dimmed(),
        pattern.description().bold()
    );
    println!("  {:<14} {}", "Mode:".dimmed(), pattern.case_mode());
    println!(
        "  {:<14} {}",
        "Attempts:".dimmed(),
        format!("~{} (avg)", estimate.attempts_label).yellow()
    );
    println!(
        "  {:<14} {}",
        "Est. time:".dimmed(),
        if estimate.risk == PatternRisk::Impractical {
            estimate.time_label.red().bold()
        } else if estimate.risk == PatternRisk::Long {
            estimate.time_label.yellow()
        } else {
            estimate.time_label.green()
        }
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
        format!(
            "~{} keys/sec on this machine (estimated)",
            format_speed(speed)
        )
        .green()
    );

    warnings::print_pattern_warnings(estimate);

    if pattern.has_suffix() || pattern.has_prefix() {
        println!();
        print_length_guide(chain.id(), estimate.pattern_chars);
    }
    println!();
}

fn print_length_guide(chain_id: &str, len: usize) {
    println!("  {}", format!("{len}-char pattern guide:").dimmed());
    let hex_chains = ["evm", "aptos", "sui", "near"];
    if hex_chains.contains(&chain_id) {
        match len {
            0..=3 => println!("    ✓ Great length — usually seconds to minutes"),
            4..=5 => println!("    ✓ OK — minutes to ~1 hour"),
            6..=7 => println!("    ⚠ Getting long — hours to days"),
            8..=9 => println!("    ⚠ Very long — days to weeks+"),
            _ => println!("    ⛔ Too long for a single machine — use ≤6 chars"),
        }
        println!("    Rule of thumb: 2→sec · 4→min · 6→~30min · 8+→hours+");
    } else {
        match len {
            0..=3 => println!("    ✓ Great length — usually seconds to minutes"),
            4..=5 => println!("    ✓ OK — minutes to ~1 hour"),
            6..=7 => println!("    ⚠ Getting long — hours to days"),
            8..=9 => println!("    ⚠ Very long — days to weeks+"),
            _ => println!("    ⛔ Too long for a single machine — use ≤6 chars"),
        }
        println!("    Rule of thumb: 2→sec · 4→min · 6→~1hr · 8+→days+");
    }
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

fn pause() {
    wait_for_key("\n  Press any key to continue...");
}

fn clear_screen() {
    // crossterm uses WinAPI on Windows (ANSI escape alone can fail in older consoles).
    let _ = crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        crossterm::cursor::MoveTo(0, 0),
    );
}
