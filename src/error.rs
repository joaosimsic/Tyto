use std::fmt;
use std::io;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io {
        path: PathBuf,
        source: io::Error,
    },
    Config {
        path: PathBuf,
        message: String,
    },
    Parse {
        source: String,
        message: String,
    },
    Validation {
        module: Option<String>,
        errors: Vec<String>,
    },
    Semantic {
        message: String,
    },
    UnsupportedLanguage {
        lang: String,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io { path, source } => {
                write!(f, "IO error for '{}': {}", path.display(), source)
            }
            Error::Config { path, message } => {
                write!(f, "Config error in '{}': {}", path.display(), message)
            }
            Error::Parse { source, message } => {
                write!(f, "Parse error in '{}': {}", source, message)
            }
            Error::Validation { module, errors } => {
                if let Some(m) = module {
                    write!(f, "Validation failed in '{}': {}", m, errors.join(", "))
                } else {
                    write!(f, "Validation failed: {}", errors.join(", "))
                }
            }
            Error::Semantic { message } => {
                write!(f, "Semantic error: {}", message)
            }
            Error::UnsupportedLanguage { lang } => {
                write!(f, "Unsupported language: {}", lang)
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl Error {
    pub fn io(path: impl Into<PathBuf>, source: io::Error) -> Self {
        Error::Io {
            path: path.into(),
            source,
        }
    }

    pub fn config(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        Error::Config {
            path: path.into(),
            message: message.into(),
        }
    }

    pub fn parse(source: impl Into<String>, message: impl Into<String>) -> Self {
        Error::Parse {
            source: source.into(),
            message: message.into(),
        }
    }

    pub fn validation(module: Option<String>, errors: Vec<String>) -> Self {
        Error::Validation { module, errors }
    }

    pub fn semantic(message: impl Into<String>) -> Self {
        Error::Semantic {
            message: message.into(),
        }
    }

    pub fn unsupported_language(lang: impl Into<String>) -> Self {
        Error::UnsupportedLanguage { lang: lang.into() }
    }
}
