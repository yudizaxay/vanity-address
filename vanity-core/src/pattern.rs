#[derive(Debug, Clone)]
pub struct Pattern {
    pub prefix: String,
    pub suffix: String,
    pub contains: String,
    pub prefix_match: String,
    pub suffix_match: String,
    pub contains_match: String,
    pub ignore_case: bool,
}

impl Pattern {
    pub fn has_prefix(&self) -> bool {
        !self.prefix.is_empty()
    }

    pub fn has_suffix(&self) -> bool {
        !self.suffix.is_empty()
    }

    pub fn has_contains(&self) -> bool {
        !self.contains.is_empty()
    }

    pub fn description(&self) -> String {
        let mut parts = Vec::new();
        if self.has_prefix() {
            parts.push(format!("starting with '{}'", self.prefix));
        }
        if self.has_contains() {
            parts.push(format!("containing '{}'", self.contains));
        }
        if self.has_suffix() {
            parts.push(format!("ending with '{}'", self.suffix));
        }
        parts.join(" and ")
    }

    pub fn case_mode(&self) -> &'static str {
        if self.ignore_case {
            "any case"
        } else {
            "exact case"
        }
    }

    /// Literal character count used for difficulty estimates (`*` wildcards ignored).
    pub fn effective_literal_chars(&self) -> usize {
        literal_len(&self.prefix) + literal_len(&self.suffix) + literal_len(&self.contains)
    }
}

/// One OR-alternative for build_pattern (prefix, suffix, contains).
pub type PatternAlternative = (Option<String>, Option<String>, Option<String>);

/// Split comma-separated OR alternatives. Only one of prefix/suffix/contains may
/// contain commas; others are shared across all alternatives.
pub fn expand_pattern_alternatives(
    prefix: Option<&str>,
    suffix: Option<&str>,
    contains: Option<&str>,
) -> Result<Vec<PatternAlternative>, String> {
    let split = |s: Option<&str>| -> Vec<String> {
        match s {
            None | Some("") => vec![],
            Some(raw) => raw
                .split(',')
                .map(|p| p.trim().to_string())
                .filter(|p| !p.is_empty())
                .collect(),
        }
    };

    let prefixes = split(prefix);
    let suffixes = split(suffix);
    let contains_list = split(contains);

    let multi_count = [
        prefixes.len() > 1,
        suffixes.len() > 1,
        contains_list.len() > 1,
    ]
    .iter()
    .filter(|&&b| b)
    .count();
    if multi_count > 1 {
        return Err("comma OR is only allowed in one of --prefix, --suffix, or --contains".into());
    }

    let n = prefixes
        .len()
        .max(suffixes.len())
        .max(contains_list.len())
        .max(1);

    if prefixes.is_empty() && suffixes.is_empty() && contains_list.is_empty() {
        return Ok(vec![(None, None, None)]);
    }

    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let p = if prefixes.len() > 1 {
            prefixes.get(i).cloned()
        } else if prefixes.len() == 1 {
            Some(prefixes[0].clone())
        } else {
            None
        };
        let s = if suffixes.len() > 1 {
            suffixes.get(i).cloned()
        } else if suffixes.len() == 1 {
            Some(suffixes[0].clone())
        } else {
            None
        };
        let c = if contains_list.len() > 1 {
            contains_list.get(i).cloned()
        } else if contains_list.len() == 1 {
            Some(contains_list[0].clone())
        } else {
            None
        };
        out.push((p, s, c));
    }
    Ok(out)
}

/// Combined expected attempts when any of several patterns may match (OR).
pub fn expected_attempts_any(per_pattern: &[f64]) -> f64 {
    if per_pattern.is_empty() {
        return f64::INFINITY;
    }
    let inv: f64 = per_pattern.iter().map(|&e| 1.0 / e.max(1.0)).sum();
    if inv <= 0.0 {
        f64::INFINITY
    } else {
        1.0 / inv
    }
}

fn literal_len(s: &str) -> usize {
    s.chars().filter(|c| *c != '*').count()
}

pub fn matches_at(address: &str, pattern: &str, ignore_case: bool, at_start: bool) -> bool {
    if pattern.is_empty() {
        return true;
    }

    if pattern.contains('*') {
        return match_glob_affix(address, pattern, ignore_case, at_start);
    }

    let addr_bytes = address.as_bytes();
    let pat_bytes = pattern.as_bytes();

    if pat_bytes.len() > addr_bytes.len() {
        return false;
    }

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

/// Glob-style match for a single `*` (zero or more chars) within an affix constraint.
fn match_glob_affix(address: &str, pattern: &str, ignore_case: bool, at_start: bool) -> bool {
    let addr = if ignore_case {
        address.to_ascii_lowercase()
    } else {
        address.to_string()
    };
    let pat = if ignore_case {
        pattern.to_ascii_lowercase()
    } else {
        pattern.to_string()
    };

    let parts: Vec<&str> = pat.split('*').collect();
    if parts.len() == 1 {
        return if at_start {
            addr.starts_with(&pat)
        } else {
            addr.ends_with(&pat)
        };
    }

    // For prefix globs: must start with first literal; for suffix: must end with last.
    if at_start {
        if !parts[0].is_empty() && !addr.starts_with(parts[0]) {
            return false;
        }
        return match_contains_glob(&addr, &pat);
    }

    if !parts[parts.len() - 1].is_empty() && !addr.ends_with(parts[parts.len() - 1]) {
        return false;
    }
    match_contains_glob(&addr, &pat)
}

fn match_contains_glob(address: &str, pattern: &str) -> bool {
    if !pattern.contains('*') {
        return address.contains(pattern);
    }
    let parts: Vec<&str> = pattern.split('*').collect();
    let mut rest = address;
    for (i, part) in parts.iter().enumerate() {
        if part.is_empty() {
            continue;
        }
        if let Some(idx) = rest.find(part) {
            if i == 0 && !pattern.starts_with('*') && idx != 0 {
                // anchored start handled by caller for prefix; for contains, first part can be anywhere
            }
            rest = &rest[idx + part.len()..];
        } else {
            return false;
        }
    }
    if !pattern.ends_with('*') {
        // last literal already consumed from rest; ok
    }
    true
}

pub fn matches_contains(address: &str, pattern: &str, ignore_case: bool) -> bool {
    if pattern.is_empty() {
        return true;
    }
    let addr = if ignore_case {
        address.to_ascii_lowercase()
    } else {
        address.to_string()
    };
    let pat = if ignore_case {
        pattern.to_ascii_lowercase()
    } else {
        pattern.to_string()
    };
    match_contains_glob(&addr, &pat)
}

pub fn matches_both(address: &str, prefix: &str, suffix: &str, ignore_case: bool) -> bool {
    matches_at(address, prefix, ignore_case, true)
        && matches_at(address, suffix, ignore_case, false)
}

pub fn matches_full(
    address: &str,
    prefix: &str,
    suffix: &str,
    contains: &str,
    ignore_case: bool,
) -> bool {
    matches_both(address, prefix, suffix, ignore_case)
        && matches_contains(address, contains, ignore_case)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_and_wildcard() {
        assert!(matches_contains("abcXYZdef", "XYZ", false));
        assert!(matches_contains("abcXYZdef", "X*Z", false));
        assert!(!matches_contains("abcXYZdef", "ZZZ", false));
        assert!(matches_at("CoolEnd", "Cool*", false, true));
        assert!(matches_at("xxpump", "*pump", false, false));
    }
}
