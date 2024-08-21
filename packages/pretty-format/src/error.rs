use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum PrettyFormatError {
    #[error("{0}")]
    Configuration(String),
}
