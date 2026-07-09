use colored::Colorize;
use vanity_core::{GrindEstimate, PatternRisk};

pub fn print_pattern_warnings(estimate: &GrindEstimate) {
    match estimate.risk {
        PatternRisk::None => {}
        PatternRisk::Caution => {
            println!();
            println!(
                "  {} {}",
                "⚠".yellow().bold(),
                "This pattern may take hours or longer. 4–6 characters is usually realistic."
                    .yellow()
            );
        }
        PatternRisk::Long => {
            println!();
            println!(
                "  {} {}",
                "⚠".red().bold(),
                "Long pattern — weeks or months on this machine. Strongly consider shortening it."
                    .red()
            );
        }
        PatternRisk::Impractical => {
            println!();
            println!(
                "  {} {}",
                "⛔".red().bold(),
                format!(
                    "{} characters is NOT practical on a single PC (years to centuries+).",
                    estimate.pattern_chars
                )
                .red()
                .bold()
            );
            println!(
                "  {}",
                "Vanity grinds are probabilistic — you will almost certainly never find a match."
                    .red()
                    .dimmed()
            );
            println!(
                "  {}",
                "Recommended: 2–4 chars suffix/prefix · 6 max for patient grinds.".dimmed()
            );
        }
    }
}
