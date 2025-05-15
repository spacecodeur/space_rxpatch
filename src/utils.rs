use std::{fs, path::Path};

use crate::InsertError;
use crate::formatters::FileType;

pub fn get_indentation(text: &str) -> Result<String, InsertError> {
    text.lines()
        .find(|line| !line.trim().is_empty())
        .and_then(|line| {
            line.find(|c: char| !c.is_whitespace())
                .map(|idx| &line[..idx])
        })
        .map(|s| s.to_string())
        .ok_or(InsertError::IndentationError)
}

pub fn file_content_to_string(file_path: &str) -> Result<String, InsertError> {
    fs::read_to_string(file_path).map_err(|e| InsertError::io_with_path(e, file_path))
}

pub fn get_file_type(file_path: &str) -> Result<FileType, InsertError> {
    let ext = Path::new(file_path)
        .extension()
        .and_then(|e| e.to_str())
        .ok_or_else(|| InsertError::unsupported_extension("", file_path))?;

    match ext.to_lowercase().as_str() {
        "env" => Ok(FileType::Env),
        "toml" => Ok(FileType::Toml),
        "rs" => Ok(FileType::Rust),
        _ => Err(InsertError::unsupported_extension(ext, file_path)),
    }
}
