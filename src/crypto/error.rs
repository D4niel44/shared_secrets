use std::error::Error;
use std::fmt;

/// An error that may occur when encrypting or decrypting.
#[derive(Debug, Clone)]
pub struct CipherError(pub String);

impl fmt::Display for CipherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CipherError {}
