use crate::pattern::Pattern;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternRisk {
    None,
    Caution,
    Long,
    Impractical,
}

impl PatternRisk {
    pub fn assess(attempts: f64, pattern_chars: usize, avg_secs: f64) -> Self {
        if attempts >= 1e15 || pattern_chars >= 10 || avg_secs >= 86400.0 * 365.0 * 10.0 {
            PatternRisk::Impractical
        } else if attempts >= 1e12 || pattern_chars >= 8 || avg_secs >= 86400.0 * 30.0 {
            PatternRisk::Long
        } else if attempts >= 1e9 || pattern_chars >= 6 || avg_secs >= 86400.0 {
            PatternRisk::Caution
        } else {
            PatternRisk::None
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
    let prefix = pattern
        .prefix_match
        .strip_prefix("0x")
        .unwrap_or(&pattern.prefix_match);
    prefix.len() + pattern.suffix_match.len()
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
            prefix_match: String::new(),
            suffix_match: suffix.to_ascii_lowercase(),
            ignore_case: true,
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
