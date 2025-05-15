mod args;
mod error;
mod formatters;
mod utils;

use args::{Config, parse_args};
use error::InsertError;

#[cfg(test)]
pub mod tests;

fn main() -> Result<(), InsertError> {
    let config: Config = parse_args();

    // Debug print of parsed configuration
    println!("Parsed configuration: {:?}", config);

    let result = rxpatch(
        &config.filepath,
        &config.replacement_type,
        &config.regex,
        &config.content,
    );

    dbg!(result);
    Ok(())
}

pub fn rxpatch(
    file_path: &str,
    replacement_type: &str,
    regex: &str,
    new_content: &str,
) -> Result<String, InsertError> {
    use regex::RegexBuilder;

    match replacement_type {
        "inline" => {
            let re = RegexBuilder::new(regex)
                .multi_line(true)
                .build()
                .map_err(|e| InsertError::parsing_failure(format!("Invalid regex: {}", e), None))?;

            let file_content = utils::file_content_to_string(file_path)?;

            // Applique la regex sur tout le texte, avec prise en compte des \n
            let result = re
                .replace_all(&file_content, |caps: &regex::Captures| {
                    if let Some(m) = caps.get(1) {
                        // Reconstitue le remplacement
                        let start = m.start();
                        let end = m.end();
                        let mut replaced = String::new();
                        replaced.push_str(&caps[0][..start - caps.get(0).unwrap().start()]);
                        replaced.push_str(new_content);
                        replaced.push_str(&caps[0][end - caps.get(0).unwrap().start()..]);
                        replaced
                    } else {
                        caps[0].to_string()
                    }
                })
                .to_string();

            Ok(result)
        }

        "block" => {
            let re = RegexBuilder::new(regex)
                .multi_line(true)
                .dot_matches_new_line(true)
                .build()
                .map_err(|e| InsertError::parsing_failure(format!("Invalid regex: {}", e), None))?;

            // let file_content = utils::file_content_to_string(file_path)?;

            // let file_type = utils::detect_file_type(file_path)?;

            // let caps = re.captures(&file_content).ok_or_else(|| {
            //     InsertError::invalid_format("file block", "No matching block found".to_string())
            // })?;

            // let match_content = caps.get(1);

            // let block_content = match_content
            //     .ok_or(InsertError::invalid_format(
            //         "regex capture",
            //         "No content captured in group 1".to_string(),
            //     ))?
            //     .as_str();

            // let indent = utils::get_indentation(block_content).unwrap_or_default();

            // Ok(insert_in_block(
            //     block_content,
            //     &format!("{}{}", indent, new_content),
            //     file_type,
            // )?)

            Ok(formatters::process(file_path, re, new_content)?)
        }
        _ => panic!("Unsupported replacement_type: {}", replacement_type),
    }
}
