use text_search::match_result::MatchResult;

use crate::Contexts;

use super::FileFormatHandler;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Context {
    CaptureInsideCommentBlock,
    MatchInsideEmptyBlock,
    MatchAfterLastBlock,
}

pub struct EnvHandler;

impl FileFormatHandler for EnvHandler {
    fn detect_specific_contexts(
        &self,
        content: &str,
        _match_result: &MatchResult,
    ) -> Vec<Contexts> {
        let contexts = Vec::new();
        let _lines: Vec<&str> = content.lines().collect();

        //...

        contexts
    }

    fn preprocess(
        &self,
        _content: &str,
        _match_result: &Option<MatchResult>,
        _contexts: &mut Vec<Contexts>,
    ) -> String {
        "".to_string()
    }

    fn inject(
        &self,
        _preprocessed: &str,
        _match_result: &Option<MatchResult>,
        _new_content: &str,
        _contexts: Vec<Contexts>,
    ) -> String {
        "pouet".to_string()
    }
}
