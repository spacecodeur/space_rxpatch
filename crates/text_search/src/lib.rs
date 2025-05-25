use error::regex_error::RegexError;
use match_result::{CaptureInfo, MatchResult, TextRange};
use regex::RegexBuilder;

pub mod match_result;

/// Analyzes the provided content using the given regular expression and returns
/// a structured match result if exactly one capturing group matches.
///
/// The function will return:
/// - `Ok(Some(MatchResult))` if the regex matches and exactly **one** capturing group is present and matched.
/// - `Ok(None)` if no match is found.
/// - `Err(RegexError)` if there are zero or more than one captured groups matched.
///
/// # Arguments
///
/// * `content` - The input string to be analyzed.
/// * `regex` - A regex string literal containing exactly one capturing group.
///
/// # Examples
///
/// ```
/// use text_search::analyze_regex;
///
/// let content = "Email: hello@example.com";
/// let regex = r"Email: (\S+@\S+\.\S+)";
/// let result = analyze_regex(content, regex).unwrap().unwrap();
///
/// assert_eq!(result.capture.content, "hello@example.com");
/// assert_eq!(result.content, "Email: hello@example.com");
/// ```
///
/// ```
/// use text_search::analyze_regex;
///
/// let content = "No email here!";
/// let regex = r"Email: (\S+@\S+\.\S+)";
/// let result = analyze_regex(content, regex).unwrap();
///
/// assert!(result.is_none());
/// ```
pub fn analyze_regex(content: &str, regex: &str) -> Result<Option<MatchResult>, RegexError> {
    let re = RegexBuilder::new(&regex)
        .multi_line(true)
        .dot_matches_new_line(true)
        .build()
        .expect("Invalid regex");

    if let Some(caps) = re.captures(content) {
        let mut captured_groups = Vec::new();
        let mut matched_group: Option<regex::Match<'_>> = None;
        let mut num_captured = 0;

        for i in 1..caps.len() {
            let group = caps.get(i);
            if let Some(m) = group {
                num_captured += 1;
                if matched_group.is_none() {
                    matched_group = Some(m);
                }
            }
            captured_groups.push(group.map(|m| m.as_str().to_string()));
        }

        if num_captured != 1 {
            return Err(RegexError::InvalidCapturedGroupsCount {
                regex_pattern: regex.to_string(),
                capture_groups: captured_groups,
            });
        }

        let matched_group = matched_group.unwrap();
        let full_match = caps.get(0).unwrap();

        Ok(Some(MatchResult {
            content: full_match.as_str().to_string(),
            range: TextRange {
                start: full_match.start(),
                end: full_match.end(),
            },
            capture: CaptureInfo {
                content: matched_group.as_str().to_string(),
                range: TextRange {
                    start: matched_group.start(),
                    end: matched_group.end(),
                },
            },
        }))
    } else {
        Ok(None)
    }
}

#[test]
fn test_multiline_regex_match_and_positions() {
    let content = "\
Hello world,
My email is user@example.com
Thank you!";

    let regex = r"My email is (\S+@\S+\.\S+)";
    let result = analyze_regex(content, regex).unwrap().unwrap();

    assert_eq!(result.content, "My email is user@example.com");
    assert_eq!(result.capture.content, "user@example.com");

    let full_range = result.range;
    let capture_range = result.capture.range;

    let expected_full_range = TextRange {
        start: content.find("My email is").unwrap(),
        end: content.find("Thank you!").unwrap() - 1, // just before \n
    };

    let expected_capture_range = TextRange {
        start: content.find("user@example.com").unwrap(),
        end: content.find("user@example.com").unwrap() + "user@example.com".len(),
    };

    assert_eq!(full_range.start, expected_full_range.start);
    assert_eq!(capture_range.start, expected_capture_range.start);
    assert_eq!(capture_range.end, expected_capture_range.end);

    let (start_line, start_col) = full_range.get_start_line_col(content);
    let (end_line, _end_col) = full_range.get_end_line_col(content);

    assert_eq!(start_line, 2); // "My email is..." is on the line 2
    assert_eq!(start_col, 1);

    // The match begin until "user@example.com", its correspond to the line 2
    assert_eq!(end_line, 2);

    let (cap_start_line, cap_start_col) = capture_range.get_start_line_col(content);
    let (cap_end_line, _cap_end_col) = capture_range.get_end_line_col(content);

    assert_eq!(cap_start_line, 2);
    dbg!(cap_start_col);
    assert!(cap_start_col == 13); // next to "My email is "
    assert_eq!(cap_end_line, 2);
}
