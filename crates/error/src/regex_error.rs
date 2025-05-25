use std::fmt;

#[derive(Debug)]
pub enum RegexError {
    /// When the regex contains more than 1 capture group with data
    InvalidCapturedGroupsCount {
        regex_pattern: String,
        capture_groups: Vec<Option<String>>,
    },
}

impl std::error::Error for RegexError {}

impl fmt::Display for RegexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCapturedGroupsCount {
                regex_pattern,
                capture_groups,
            } => {
                let captured_values: Vec<_> = capture_groups
                    .iter()
                    .enumerate()
                    .map(|(i, val)| match val {
                        Some(v) => format!("Group {}: '{}'", i, v),
                        None => format!("Group {}: <None>", i),
                    })
                    .collect();
                write!(
                    f,
                    "RegexError: expected exactly one captured group with data, \
                    but got multiple or none.\nPattern: '{}'\nCaptures:\n{}",
                    regex_pattern,
                    captured_values.join("\n")
                )
            }
        }
    }
}

// Convenience constructors for errors
impl RegexError {
    pub fn invalid_captured_groups_count(
        regex_pattern: &'static str,
        capture_groups: Vec<Option<String>>,
    ) -> Self {
        Self::InvalidCapturedGroupsCount {
            regex_pattern: regex_pattern.to_string(),
            capture_groups,
        }
    }
}
