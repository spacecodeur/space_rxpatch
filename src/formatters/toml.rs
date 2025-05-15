use regex::Regex;

use crate::error::InsertError;

pub fn process(file_content: String, re: Regex, new_line: &str) -> Result<String, InsertError> {
    Ok("coucou".to_string())
}

pub fn insert_into_block(block_content: &str, new_item: &str) -> Result<String, InsertError> {
    // Parse existing items
    let mut items: Vec<String> = block_content
        .split(',')
        .map(|item| item.trim().trim_matches('"').to_string())
        .filter(|item| !item.is_empty())
        .collect();

    // Add new item
    items.push(new_item.trim().to_string());

    // Sort alphabetically
    items.sort();

    // Rebuild array content
    let result = items
        .iter()
        .map(|item| format!("\"{}\"", item))
        .collect::<Vec<_>>()
        .join(", ");

    Ok(result)
}
