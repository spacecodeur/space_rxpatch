use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum FileType {
    Env,
    Json,
    Java,
    Dockerfile,
    Typescript,
    Unknown,
}

impl FileType {
    pub fn from_str(format: &str) -> FileType {
        match format.to_lowercase().as_str() {
            "env" => FileType::Env,
            "json" => FileType::Json,
            "java" => FileType::Java,
            "dockerfile" => FileType::Dockerfile,
            "ts" | "typescript" => FileType::Typescript,
            _ => FileType::Unknown,
        }
    }

    pub fn from_path(path_str: &str, file_force_format: Option<String>) -> FileType {
        if let Some(format) = file_force_format {
            return FileType::from_str(&format);
        }

        let path = Path::new(path_str);

        if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
            if file_name.eq_ignore_ascii_case("Dockerfile") {
                return FileType::Dockerfile;
            }
        }

        if Self::is_env_file(path) {
            return FileType::Env;
        }

        match path.extension().and_then(|s| s.to_str()) {
            Some(ext) => FileType::from_str(ext),
            None => FileType::Unknown,
        }
    }

    fn is_env_file(path: &Path) -> bool {
        if !path.is_file() {
            return false;
        }

        let filename = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name,
            None => return false,
        };

        if !filename.starts_with(".env") {
            return false;
        }

        if filename == ".env" {
            return true;
        }

        matches!(filename.chars().nth(4), Some('.'))
    }
}
