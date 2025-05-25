pub mod dockerfile_handler;
pub mod env_handler;
pub mod json_handler;

use dockerfile_handler::DockerfileHandler;
use env_handler::EnvHandler;
use json_handler::JsonHandler;
use text_search::match_result::MatchResult;

use crate::{Contexts, file_type::FileType};

/// Trait implemented per file format
pub trait FileFormatHandler {
    fn detect_specific_contexts(&self, content: &str, match_result: &MatchResult) -> Vec<Contexts>;

    fn preprocess(
        &self,
        content: &str,
        match_result: &Option<MatchResult>,
        contexts: &mut Vec<Contexts>,
    ) -> String;

    fn inject(
        &self,
        preprocessed: &str,
        match_result: &Option<MatchResult>,
        new_content: &str,
        contexts: Vec<Contexts>,
    ) -> String;
}

/// Dispatcher function
pub fn get_handler_for_type(file_type: &FileType) -> Box<dyn FileFormatHandler> {
    match file_type {
        FileType::Json => Box::new(JsonHandler),
        FileType::Dockerfile => Box::new(DockerfileHandler),
        FileType::Env => Box::new(EnvHandler),
        _ => panic!("Unsupported file type"),
    }
}
