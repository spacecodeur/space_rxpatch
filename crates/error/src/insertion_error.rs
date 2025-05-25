use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum InsertError {
    /// When the file type is not supported
    UnsupportedFileType {
        extension: Option<String>,
        filepath: Option<PathBuf>,
    },
    /// When the content format is invalid
    InvalidFormat {
        expected: &'static str,
        context: String,
    },
    /// Syntax parsing error
    ParsingError { reason: String, line: Option<usize> },
    /// Indentation detection problem
    IndentationError,
    /// Input/output error
    IoError {
        source: std::io::Error,
        path: Option<PathBuf>,
    },
    /// Validation error
    ValidationError { rule: &'static str, details: String },
}

impl fmt::Display for InsertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedFileType {
                extension,
                filepath,
            } => {
                if let Some(ext) = extension {
                    write!(f, "Unsupported file extension '{}'", ext)?;
                } else {
                    write!(f, "Unsupported file type")?;
                }

                if let Some(path) = filepath {
                    write!(f, " (file: {})", path.display())?;
                }
                Ok(())
            }
            Self::InvalidFormat { expected, context } => {
                write!(
                    f,
                    "Invalid format. Expected: {}. Context: {}",
                    expected, context
                )
            }
            Self::ParsingError { reason, line } => {
                write!(f, "Parsing error: {}", reason)?;
                if let Some(l) = line {
                    write!(f, " at line {}", l)?;
                }
                Ok(())
            }
            Self::IndentationError => {
                write!(f, "Failed to detect indentation")
            }
            Self::IoError { source, path } => {
                write!(f, "I/O error: {}", source)?;
                if let Some(p) = path {
                    write!(f, " (path: {})", p.display())?;
                }
                Ok(())
            }
            Self::ValidationError { rule, details } => {
                write!(f, "Validation rule '{}' violated: {}", rule, details)
            }
        }
    }
}

impl std::error::Error for InsertError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::IoError { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl From<std::io::Error> for InsertError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError {
            source: err,
            path: None,
        }
    }
}

// Convenience constructors for errors
impl InsertError {
    pub fn unsupported_extension(ext: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self::UnsupportedFileType {
            extension: Some(ext.into()),
            filepath: Some(path.into()),
        }
    }

    pub fn invalid_format(expected: &'static str, context: impl Into<String>) -> Self {
        Self::InvalidFormat {
            expected,
            context: context.into(),
        }
    }

    pub fn parsing_failure(reason: impl Into<String>, line: Option<usize>) -> Self {
        Self::ParsingError {
            reason: reason.into(),
            line,
        }
    }

    pub fn io_with_path(err: std::io::Error, path: impl Into<PathBuf>) -> Self {
        Self::IoError {
            source: err,
            path: Some(path.into()),
        }
    }
}
