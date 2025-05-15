use clap::{Arg, ArgAction, Command};
use std::process;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub filepath: String,
    pub regex: String,
    pub replacement_type: String,
    pub content: String,
    pub suffix: Option<String>,
    pub dry_run: bool,
}

/// Display an error and exit if regex_type is invalid
fn validate_config(config: &Config) {
    if config.replacement_type != "inline" && config.replacement_type != "block" {
        eprintln!("Error: --regex-type must be either 'inline' or 'block'");
        process::exit(1);
    }
}

pub fn parse_args() -> Config {
    let matches = Command::new("rxpatch")
        .about("Apply regex-based patching to a text file")
        .arg(
            Arg::new("filepath")
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
            Arg::new("replacement-type")
                .short('t')
                .long("replacement-type")
                .value_name("TYPE")
                .help("Replacement type: inline or block")
                .required(true)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("content")
                .short('c')
                .long("content")
                .value_name("CONTENT")
                .help("Replacement content")
                .required(true)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("suffix")
                .short('s')
                .long("suffix")
                .value_name("SUFFIX")
                .help("Optional suffix to add")
                .required(false)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("dry-run")
                .long("dry-run")
                .help("Perform a trial run with no changes made")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let config = Config {
        filepath: matches.get_one::<String>("filepath").unwrap().to_string(),
        regex: matches.get_one::<String>("regex").unwrap().to_string(),
        replacement_type: matches
            .get_one::<String>("replacement-type")
            .unwrap()
            .to_string(),
        content: matches.get_one::<String>("content").unwrap().to_string(),
        suffix: matches.get_one::<String>("suffix").map(|s| s.to_string()),
        dry_run: matches.get_flag("dry-run"),
    };

    validate_config(&config);
    config
}
