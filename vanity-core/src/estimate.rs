use crate::pattern::Pattern;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternRisk {
    None,
    Caution,
    Long,
    Impractical,
}

impl PatternRisk {
    /// Judged on time, not length: 8 hex characters take hours, 8 base58 ones take years.
    pub fn assess(attempts: f64, _pattern_chars: usize, avg_secs: f64) -> Self {
        const DAY: f64 = 86_400.0;
        if attempts >= 1e15 || avg_secs >= DAY * 365.0 * 10.0 {
            PatternRisk::Impractical
        } else if avg_secs >= DAY * 7.0 {
            PatternRisk::Long
        } else if avg_secs >= 3_600.0 {
            PatternRisk::Caution
        } else {
            PatternRisk::None
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            PatternRisk::None => "none",
            PatternRisk::Caution => "caution",
            PatternRisk::Long => "long",
            PatternRisk::Impractical => "impractical",
        }
    }
}

#[derive(Debug, Clone)]
pub struct GrindEstimate {
    pub attempts: f64,
    pub attempts_label: String,
    pub avg_secs: f64,
    pub time_label: String,
    pub difficulty: &'static str,
    pub difficulty_bars: String,
    pub risk: PatternRisk,
    pub pattern_chars: usize,
}

pub fn effective_pattern_chars(pattern: &Pattern) -> usize {
    pattern.effective_literal_chars()
}

pub fn format_attempts(n: f64) -> String {
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

pub fn format_duration(avg_secs: f64) -> String {
    if avg_secs < 5.0 {
        "a few seconds".to_string()
    } else if avg_secs < 60.0 {
        format!("~{:.0} seconds", avg_secs)
    } else if avg_secs < 3_600.0 {
        format!("~{:.0} minutes", avg_secs / 60.0)
    } else if avg_secs < 86_400.0 {
        format!("~{:.1} hours", avg_secs / 3_600.0)
    } else if avg_secs < 86_400.0 * 30.0 {
        format!("~{:.0} days", avg_secs / 86_400.0)
    } else if avg_secs < 86_400.0 * 365.0 {
        format!("~{:.0} months", avg_secs / (86_400.0 * 30.0))
    } else if avg_secs < 86_400.0 * 365.0 * 100.0 {
        format!("~{:.0} years", avg_secs / (86_400.0 * 365.0))
    } else {
        "centuries+ — not practical on one machine".to_string()
    }
}

/// Single-thread heuristic throughput when no live benchmark is available (wasm / SDK).
pub fn default_keys_per_sec(chain_id: &str) -> f64 {
    match chain_id {
        "evm" | "eth" | "ethereum" | "aptos" | "apt" | "sui" | "near" => 35_000.0,
        _ => 80_000.0,
    }
}

/// Verdict for the estimated time, and what one character more or less would do.
/// `per_char` is how many options each pattern character has (see `Chain::chars_per_position`).
pub fn pattern_guide(estimate: &GrindEstimate, per_char: f64) -> (&'static str, String) {
    const DAY: f64 = 86_400.0;
    let verdict = match estimate.avg_secs {
        s if s < 60.0 => "✓ Great — about a minute or less",
        s if s < 3_600.0 => "✓ OK — minutes",
        s if s < DAY => "⚠ Getting long — hours",
        s if s < DAY * 30.0 => "⚠ Very long — days",
        _ => "⛔ Too long for one machine — shorten the pattern",
    };
    let per_char = per_char.round();
    let shorter = format_duration(estimate.avg_secs / per_char);
    let rule = if estimate.pattern_chars > 1 {
        format!("Each character ≈ {per_char:.0}× longer · one fewer ≈ {shorter}")
    } else {
        format!("Each character ≈ {per_char:.0}× longer")
    };
    (verdict, rule)
}

pub fn grind_estimate(attempts: f64, keys_per_sec: f64, pattern: &Pattern) -> GrindEstimate {
    let keys_per_sec = keys_per_sec.max(1.0);
    let avg_secs = attempts / keys_per_sec;
    let pattern_chars = effective_pattern_chars(pattern);
    let risk = PatternRisk::assess(attempts, pattern_chars, avg_secs);

    let (difficulty, filled) = match attempts {
        a if a < 100_000.0 => ("Easy", 2),
        a if a < 10_000_000.0 => ("Quick", 3),
        a if a < 1_000_000_000.0 => ("Medium", 5),
        a if a < 1_000_000_000_000.0 => ("Hard", 7),
        _ => ("Extreme", 10),
    };

    GrindEstimate {
        attempts,
        attempts_label: format_attempts(attempts),
        avg_secs,
        time_label: format_duration(avg_secs),
        difficulty,
        difficulty_bars: format!("{}{}", "█".repeat(filled), "░".repeat(10 - filled)),
        risk,
        pattern_chars,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::Pattern;

    fn suffix_pattern(suffix: &str) -> Pattern {
        Pattern {
            prefix: String::new(),
            suffix: suffix.to_string(),
            contains: String::new(),
            prefix_match: String::new(),
            suffix_match: suffix.to_ascii_lowercase(),
            contains_match: String::new(),
            ignore_case: true,
            fixed_prefix_len: 0,
        }
    }

    #[test]
    fn long_suffix_is_impractical() {
        let p = suffix_pattern("akshaysingh");
        let est = grind_estimate(1e16, 720_000.0, &p);
        assert_eq!(est.risk, PatternRisk::Impractical);
    }

    #[test]
    fn short_suffix_is_easy() {
        let p = suffix_pattern("ab");
        let est = grind_estimate(3_364.0, 720_000.0, &p);
        assert_eq!(est.risk, PatternRisk::None);
    }
}
