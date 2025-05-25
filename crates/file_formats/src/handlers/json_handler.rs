use text_search::match_result::MatchResult;

use crate::Contexts;

use super::FileFormatHandler;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Context {
    CaptureInsideCurlyBrackets,
    CaptureInsideSquareBrackets,
}

pub struct JsonHandler;

impl FileFormatHandler for JsonHandler {
    fn detect_specific_contexts(&self, content: &str, match_result: &MatchResult) -> Vec<Contexts> {
        let mut contexts = Vec::new();

        // Get the surrounding text before and after the captured group
        let before = content[..match_result.capture.range.start].trim_end();
        let after = content[match_result.capture.range.end..].trim_start();

        // Get the last character before the capture (if any)
        let char_before = before.chars().rev().next();
        // Get the first character after the capture (if any)
        let char_after = after.chars().next();

        if char_before == Some('{') && char_after == Some('}') {
            contexts.push(Contexts::JSON(Context::CaptureInsideCurlyBrackets));
        }

        contexts
    }

    fn preprocess(
        &self,
        content: &str,
        _match_result: &Option<MatchResult>,
        _contexts: &mut Vec<Contexts>,
    ) -> String {
        content.to_string()
    }

    fn inject(
        &self,
        preprocessed: &str,
        _match_result: &Option<MatchResult>,
        _new_content: &str,
        _contexts: Vec<Contexts>,
    ) -> String {
        let result = preprocessed.to_string();
        // result.insert_str(match_result.capture_range.end, new_content);
        result
    }
}
