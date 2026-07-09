#[derive(Debug, Clone)]
pub struct Pattern {
    pub prefix: String,
    pub suffix: String,
    pub prefix_match: String,
    pub suffix_match: String,
    pub ignore_case: bool,
}

impl Pattern {
    pub fn has_prefix(&self) -> bool {
        !self.prefix.is_empty()
    }

    pub fn has_suffix(&self) -> bool {
        !self.suffix.is_empty()
    }

    pub fn description(&self) -> String {
        match (self.has_prefix(), self.has_suffix()) {
            (true, true) => format!(
                "starting with '{}' and ending with '{}'",
                self.prefix, self.suffix
            ),
            (true, false) => format!("starting with '{}'", self.prefix),
            (false, true) => format!("ending with '{}'", self.suffix),
            (false, false) => String::new(),
        }
    }

    pub fn case_mode(&self) -> &'static str {
        if self.ignore_case {
            "any case"
        } else {
            "exact case"
        }
    }
}

pub fn matches_at(address: &str, pattern: &str, ignore_case: bool, at_start: bool) -> bool {
    if pattern.is_empty() {
        return true;
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

pub fn matches_both(address: &str, prefix: &str, suffix: &str, ignore_case: bool) -> bool {
    matches_at(address, prefix, ignore_case, true)
        && matches_at(address, suffix, ignore_case, false)
}
