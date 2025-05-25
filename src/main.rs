mod args;
mod file;

use args::parse_args;
use file::read_file;
use file_formats::{
    Contexts,
    common_context::{Context, detect_common_contexts},
    file_type::FileType,
    handlers::get_handler_for_type,
};
use text_search::analyze_regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 0: Parse CLI arguments
    let config = parse_args();

    // Step 1: Read input file and determine file type
    let original_content = read_file(&config.file_path)?;
    let file_type = FileType::from_path(&config.file_path, config.file_force_format);

    // Step 2: Run regex on file content
    let match_result = analyze_regex(&original_content, &config.regex)?;

    dbg!(&config.regex);
    dbg!(&match_result);

    // Step 3: Detect common and specific contexts
    let mut contexts: Vec<Contexts> = detect_common_contexts(&original_content, &match_result);

    let handler = get_handler_for_type(&file_type);

    if contexts.contains(&Contexts::Common(Context::MatchIsFound)) {
        let specific_contexts =
            handler.detect_specific_contexts(&original_content, &match_result.clone().unwrap());
        contexts.extend(specific_contexts);
    }

    // Step 4: Preprocess file content based on contexts
    let preprocessed = handler.preprocess(&original_content, &match_result, &mut contexts);

    // Step 5: Inject new content at the correct location
    let final_content = handler.inject(&preprocessed, &match_result, &config.new_content, contexts);

    // Step 6: Write updated content back to the file
    if config.write == false {
        println!("{}", final_content);
    } else {
        // write_file(&config.file_path, &final_content);
    }

    Ok(())
}
