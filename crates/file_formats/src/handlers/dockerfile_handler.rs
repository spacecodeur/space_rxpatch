use text_search::match_result::MatchResult;

use crate::Contexts;

use super::FileFormatHandler;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Context {
    KeyHasMultipleValues,
}

pub struct DockerfileHandler;

impl FileFormatHandler for DockerfileHandler {
    fn detect_specific_contexts(
        &self,
        _content: &str,
        _match_result: &MatchResult,
    ) -> Vec<Contexts> {
        // Dummy impl
        vec![]
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
