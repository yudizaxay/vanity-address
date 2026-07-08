use colored::Colorize;
use std::io::{self, Write};

/// Full splash — title and trust taglines (main menu).
pub fn print_splash() {
    println!();
    print_title_block();
    print_trust_lines();
    println!();
}

/// Compact header for wizard steps.
pub fn print_compact() {
    println!();
    println!(
        "  {}  {}",
        "vanity-address".cyan().bold(),
        "v0.2.0".dimmed()
    );
    println!(
        "  {}",
        "local · offline · keys never leave your machine".dimmed()
    );
    println!();
}

fn print_title_block() {
    println!(
        "  {}",
        "╔══════════════════════════════════════════╗".cyan()
    );
    println!(
        "  {}",
        "║      vanity-address  ·  v0.2.0            ║"
            .cyan()
            .bold()
    );
    println!(
        "  {}",
        "║   multi-chain vanity address generator    ║".cyan()
    );
    println!(
        "  {}",
        "╚══════════════════════════════════════════╝".cyan()
    );
}

fn print_trust_lines() {
    println!();
    println!(
        "  {} {}",
        "·".cyan(),
        "Keys generated & stored on this machine only".dimmed()
    );
    println!(
        "  {} {}",
        "·".cyan(),
        "No internet — fully offline, nothing ever uploaded".dimmed()
    );
    println!(
        "  {} {}",
        "·".cyan(),
        "13 chains  ·  prefix/suffix  ·  multi-core grinding".dimmed()
    );
    let _ = io::stdout().flush();
}
