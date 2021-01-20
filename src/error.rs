use std::error::Error;

/// An error that may occur when parsing arguments
#[derive(Debug, Clone)]
pub struct ArgumentError(pub String);

impl std::fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ArgumentError {}

/// An error that indicates that the shares file is corrupt
#[derive(Debug, Clone)]
pub struct CorruptFileError(pub String);

impl std::fmt::Display for CorruptFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CorruptFileError {}
