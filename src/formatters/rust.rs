use regex::Regex;

use crate::error::InsertError;

pub fn process(file_content: String, re: Regex, new_line: &str) -> Result<String, InsertError> {
    Ok("coucou".to_string())
}

pub fn insert_into_block(block_content: &str, new_item: &str) -> Result<String, InsertError> {
    // Parse existing items
    let mut items: Vec<String> = block_content
        .split(',')
        .map(|item| item.trim().trim_end_matches(',').to_string())
        .filter(|item| !item.is_empty())
        .collect();

    // Add new item
    items.push(new_item.trim().to_string());

    // Sort alphabetically (case-sensitive)
    items.sort();

    // Rebuild block content with commas
    let result = items
        .iter()
        .map(|item| item.to_string())
        .collect::<Vec<_>>()
        .join(",\n");

    Ok(result)
}
