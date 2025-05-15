use regex::Regex;

use crate::{error::InsertError, utils};

pub mod env;
pub mod rust;
pub mod toml;

#[derive(Debug, Clone, Copy)]
pub enum FileType {
    Env,
    Toml,
    Rust,
}

pub fn process(file_path: &str, re: Regex, new_line: &str) -> Result<String, InsertError> {
    let file_content = utils::file_content_to_string(file_path)?;

    match utils::get_file_type(file_path)? {
        FileType::Env => env::process(file_content, re, new_line),
        FileType::Toml => toml::process(file_content, re, new_line),
        FileType::Rust => rust::process(file_content, re, new_line),
    }
}

#[derive(Debug)]
pub struct Block {
    start_num_line_in_input: usize,
    start_num_col_in_input: usize,
    content: String,
}

impl Block {
    fn new(input: &str, regex: Regex) -> Result<Block, InsertError> {
        if let Some(caps) = regex.captures(input) {
            if caps.len() > 2 {
                return Err(InsertError::invalid_format(
                    "regex with less than 2 capture groups",
                    "The regex pattern must not contain more than two capture groups".to_string(),
                ));
            }

            let mut block = Block {
                start_num_line_in_input: 0,
                start_num_col_in_input: 0,
                content: String::new(),
            };

            if let Some(mat) = caps.get(1) {
                let start = mat.start();
                let end = mat.end();

                // Découper le texte jusqu'au début de la capture
                let prefix = &input[..start];
                let capture = &input[start..end];

                // Calcul de la ligne et colonne
                let line_number = prefix.matches('\n').count() + 1;
                let col_number = prefix
                    .rfind('\n')
                    .map(|pos| start - pos)
                    .unwrap_or(start + 1); // Si aucune newline avant => colonne = start + 1

                block.content = mat.as_str().to_string();
                block.start_num_line_in_input = line_number;
                block.start_num_col_in_input = col_number;

                Ok(block)
            } else {
                let full_match = caps.get(0).unwrap();

                let end = full_match.end();
                let prefix = &input[..end];

                let line_number = prefix.matches('\n').count() + 1;
                let col_number = prefix.rfind('\n').map(|pos| end - pos).unwrap_or(end + 1);

                block.start_num_line_in_input = line_number;
                block.start_num_col_in_input = col_number;

                Ok(block)
            }
        } else {
            Err(InsertError::invalid_format(
                "Regex with less than 2 capture groups",
                "No matches found for the regex".to_string(),
            ))
        }
    }

    fn count_lines(&self) -> usize {
        self.content.lines().count()
    }
}

fn insert_at(
    file_content: &str,
    mut target_line: usize,
    target_col: usize,
    to_insert: &str,
) -> String {
    println!("insert_at >>>");
    dbg!(file_content);
    dbg!(target_line);
    dbg!(target_col);
    dbg!(to_insert);
    println!("<<< insert_at");

    let mut lines: Vec<String> = file_content.lines().map(|line| line.to_string()).collect();

    // S'assurer que la ligne existe
    if lines.is_empty() {
        lines.push(String::new());
    }

    if lines.len() < target_line {
        target_line = lines.len();
        lines.push(String::new());
    }

    // Obtenir la ligne ciblée
    let line = &mut lines[target_line];

    // Insertion dans la ligne ciblée
    line.insert_str(target_col, to_insert);

    // Reconstruire le contenu du fichier
    let result = lines.join("\n");

    println!("result :");
    dbg!(&result);

    result
}

// pub fn capture_position(input: &str, regex: &Regex) -> Result<Block, InsertError> {
//     if let Some(caps) = regex.captures(input) {
//         if caps.len() > 2 {
//             return Err(InsertError::invalid_format(
//                 "regex with less than 2 capture groups",
//                 "The regex pattern must not contain more than two capture groups".to_string(),
//             ));
//         }

//         let mut block = Block {
//             start_num_line_in_input: 0,
//             start_num_col_in_input: 0,
//             nb_lines: 0,
//         };

//         if let Some(mat) = caps.get(1) {
//             let start = mat.start();
//             let end = mat.end();

//             // Découper le texte jusqu'au début de la capture
//             let prefix = &input[..start];
//             let capture = &input[start..end];

//             // Calcul de la ligne et colonne
//             let line_number = prefix.matches('\n').count() + 1;
//             let col_number = prefix
//                 .rfind('\n')
//                 .map(|pos| start - pos)
//                 .unwrap_or(start + 1); // Si aucune newline avant => colonne = start + 1

//             // Nombre de lignes capturées
//             let lines_captured = capture.matches('\n').count() + 1;

//             block.nb_lines = lines_captured as i16;
//             block.start_num_line_in_input = line_number as i16;
//             block.start_num_col_in_input = col_number as i16;

//             Ok(block)
//         } else {
//             let full_match = caps.get(0).unwrap();

//             let end = full_match.end();
//             let prefix = &input[..end];

//             let line_number = prefix.matches('\n').count() + 1;
//             let col_number = prefix.rfind('\n').map(|pos| end - pos).unwrap_or(end + 1);

//             block.start_num_line_in_input = line_number as i16;
//             block.start_num_col_in_input = col_number as i16;

//             Ok(block)
//         }
//     } else {
//         Err(InsertError::invalid_format(
//             "Regex with less than 2 capture groups",
//             "No matches found for the regex".to_string(),
//         ))
//     }
// }
