use crate::Contexts;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Context {
    FileIsEmpty,
    FileIsNotEmpty,
    MatchBeginAtLastLine,
    CaptureGroupIsEmpty,
    CaptureGroupIsNotEmpty,
    MatchIsNotFound,
    MatchIsFound,
}

/// Collect common contexts here, or re-export a detection utility
pub fn detect_common_contexts(
    content: &str,
    match_result: &Option<text_search::match_result::MatchResult>,
) -> Vec<Contexts> {
    let mut contexts: Vec<Contexts> = Vec::new();

    if match_result.is_none() {
        contexts.push(Contexts::Common(Context::MatchIsNotFound));
        return contexts;
    }

    contexts.push(Contexts::Common(Context::MatchIsFound));

    if content.trim().is_empty() {
        contexts.push(Contexts::Common(Context::FileIsEmpty));
    } else {
        contexts.push(Contexts::Common(Context::FileIsNotEmpty));
    }

    if match_result
        .clone()
        .unwrap()
        .capture
        .content
        .trim()
        .is_empty()
    {
        contexts.push(Contexts::Common(Context::CaptureGroupIsEmpty));
    } else {
        contexts.push(Contexts::Common(Context::CaptureGroupIsNotEmpty));
    }

    // etc. Add other common contexts...

    contexts
}
