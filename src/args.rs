use clap::{Arg, ArgAction, Command};
// use std::process;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub file_path: String,
    pub regex: String,
    // pub replacement_type: String,
    pub new_content: String,
    pub write: bool,
    pub file_force_format: Option<String>,
}

/// Display an error and exit if regex_type is invalid
fn validate_config(_config: &Config) {
    // if config.replacement_type != "inline" && config.replacement_type != "block" {
    //     eprintln!("Error: --regex-type must be either 'inline' or 'block'");
    //     process::exit(1);
    // }
}

pub fn parse_args() -> Config {
    let matches = Command::new("rxpatch")
        .about("Apply regex-based patching to a text file")
        .arg(
            Arg::new("file-path")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Path to the input file")
                .required(true)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("regex")
                .short('r')
                .long("regex")
                .value_name("REGEX")
                .help("Regex pattern to match")
                .required(true)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("new-content")
                .short('c')
                .long("new-content")
                .value_name("CONTENT")
                .help("Replacement content")
                .required(true)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("write")
                .long("write")
                .help("write the result to the file")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("file-force-format")
                .long("file-force-format")
                .value_name("FORMAT")
                .help("Force interpretation of the input file as a specific format (e.g., 'env', 'Dockerfile', 'json', ...)")
                .required(false)
                .action(ArgAction::Set),
        )
        .get_matches();

    let config = Config {
        file_path: matches.get_one::<String>("file-path").unwrap().to_string(),
        regex: matches.get_one::<String>("regex").unwrap().to_string(),
        // replacement_type: matches
        //     .get_one::<String>("replacement-type")
        //     .unwrap()
        //     .to_string(),
        new_content: matches
            .get_one::<String>("new-content")
            .unwrap()
            .to_string(),
        write: matches.get_flag("write"),
        file_force_format: matches
            .get_one::<String>("file-force-format")
            .map(|s| s.to_string()),
    };

    validate_config(&config);
    config
}
