use std::error::Error;

use rug::Integer;

use crate::math::error::ValueError;

/// Simple struct for representing prime numbers to use with modular
/// integers.
///
/// The objective of this type is to provide a unique instance from which
/// references can be borrowed when creating and manipulating modular integers.
#[derive(Debug, Eq)]
pub struct Prime {
    pub(in crate::math) value: Integer,
}

impl Prime {
    /// Parses the number represented by the string and returns it
    /// as a Prime.
    ///
    /// Requires the string to represent a number greater or equal to 2.
    ///
    /// # Errors
    ///
    /// This methods returns an error if an error occurs while parsing
    /// or if the number represented by the given string is lower than 2.
    ///
    /// # Notes
    ///
    /// This methods only wraps the given string as a Prime number,
    /// this method does not check if the integer represented by
    /// this string is actually a prime.
    pub fn parse(string: &str) -> Result<Self, Box<dyn Error>> {
        let value = Integer::from(Integer::parse(string)?);
        if value <= 1 {
            Err(Box::new(ValueError(
                "Expected a value greater than 1".into(),
            )))
        } else {
            Ok(Prime { value })
        }
    }
}

impl PartialEq for Prime {
    /// Returns True if these prime number equals other.
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

/// Auxiliar struct to help having unique primes across programs.
#[derive(Debug)]
pub struct CachedPrime {
    value: Option<Prime>,
    value_rep: &'static str,
}

impl CachedPrime {
    /// Returns the prime stored in this struct, computing it
    /// only if neccessary.
    pub fn value(&mut self) -> &Prime {
        if self.value == None {
            self.value = Some(Prime::parse(&self.value_rep).unwrap());
        }
        self.value.as_ref().unwrap()
    }
}

pub static PRIME_257_BITS: CachedPrime = CachedPrime {
    value: None,
    value_rep:
        "20835161_7316091241_2343267463_1212444825_1235562226_4704915141_8633121705_0270460481",
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_parse_ok() {
        let mut prime = "31";
        Prime::parse(&prime).unwrap();
        prime = "2";
        Prime::parse(&prime).unwrap();
    }

    #[test]
    fn prime_parse_err() -> Result<(), String> {
        match Prime::parse("-1") {
            Ok(_) => return Err("expected to return error".into()),
            _ => (),
        };
        match Prime::parse("1") {
            Ok(_) => return Err("expected to return error".into()),
            _ => (),
        };
        match Prime::parse("0") {
            Ok(_) => Err("expected to return error".into()),
            _ => Ok(()),
        }
    }
}
