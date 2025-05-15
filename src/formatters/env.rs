use regex::Regex;

use crate::error::InsertError;

use super::{Block, insert_at};

pub fn process(
    mut file_content: String,
    re: Regex,
    new_content: &str,
) -> Result<String, InsertError> {
    let mut block = Block::new(&file_content, re)?;

    (file_content, block) = preprocess_file_content(file_content, block);

    block.content = insert_into_block(&block.content, new_content)?;

    file_content = insert_at(
        &file_content,
        block.start_num_line_in_input,
        block.start_num_col_in_input,
        &block.content,
    );

    // block

    Ok(file_content)
}

fn preprocess_file_content(mut file_content: String, mut block: Block) -> (String, Block) {
    println!("AAAAA >>>");
    dbg!(&file_content);
    dbg!(&block);

    // pas de contenu capturé : cas ou on chope juste le nom de la section

    //

    if block.count_lines() == 0 {
        // block matched is empty

        // file_content = insert_at(
        //     &file_content,
        //     block.start_num_line_in_input,
        //     block.start_num_col_in_input,
        //     "\n",
        // );

        block.start_num_col_in_input = 0;
        block.start_num_line_in_input += 1;
    }

    (file_content, block)
}

// fn preprocess(file_content: String, block: Block) -> Result<(String, Block), InsertError> {
//     let lignes: Vec<String> = texte.lines().map(|s| s.to_string()).collect();

//     let mut block: Block = Block {
//         content: (),
//         line_start: (),
//         line_end: (),
//         nbr_lines_added: (),
//     };

//     let caps = re.captures(&file_content).ok_or_else(|| {
//         InsertError::invalid_format("file block", "No matching block found".to_string())
//     })?;

//     if caps.get(1).is_some() {
//         Err(InsertError::invalid_format(
//             "regex (env file)",
//             "The regex pattern must not contain capture group for .env file, only section name is needed".to_string(),
//         ));
//     }

//     let match_content = caps.get(1);

//     let block_content = match_content
//         .ok_or(InsertError::invalid_format(
//             "regex capture",
//             "No content captured in group 1".to_string(),
//         ))?
//         .as_str();

//     let indent = utils::get_indentation(block_content).unwrap_or_default();

//     Ok(insert_in_block(
//         block_content,
//         &format!("{}{}", indent, new_content),
//         file_type,
//     )?)
// }

pub fn insert_into_block(block_content: &str, new_content: &str) -> Result<String, InsertError> {
    // Parse existing entries
    let mut entries: Vec<&str> = block_content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();

    // Add new entry
    entries.push(new_content.trim());

    // Sort alphabetically by key
    entries.sort_by_key(|entry| entry.split('=').next().unwrap_or(""));

    // Rebuild block content
    Ok(entries.join("\n"))
}
