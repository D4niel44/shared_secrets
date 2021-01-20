use std::error::Error;
use std::fmt;

pub use rug::integer::ParseIntegerError;

/// This error is returned when the value
/// represented by an argument in a function
/// call is illegal or invalid.
#[derive(Debug, Clone)]
pub struct ValueError(pub String);

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ValueError {}
