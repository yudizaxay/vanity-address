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
                "This pattern may take hours or longer. Dropping a character or two makes it much faster."
                    .yellow()
            );
        }
        PatternRisk::Long => {
            println!();
            println!(
                "  {} {}",
                "⚠".red().bold(),
                "Long pattern — a week or more on this machine. Strongly consider shortening it."
                    .red()
            );
        }
        PatternRisk::Impractical => {
            println!();
            println!(
                "  {} {}",
                "⛔".red().bold(),
                format!(
                    "{} characters is NOT practical on a single PC ({}).",
                    estimate.pattern_chars, estimate.time_label
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
                "Shorten the pattern until the estimate is hours, not years.".dimmed()
            );
        }
    }
}
