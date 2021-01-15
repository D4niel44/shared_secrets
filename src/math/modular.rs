use std::error::Error;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

use rug::ops::RemRounding;
use rug::Integer;

use crate::math::error::{ParseIntegerError, ValueError};
use crate::math::field::Field;
use crate::math::random::Rng;

/// Simple struct for representing prime numbers to use with modular
/// integers.
///
/// The objective of this type is to provide a unique instance from which
/// references can be borrowed when creating and manipulating modular integers.
#[derive(Debug, Eq)]
pub struct Prime {
    value: Integer,
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
    fn parse(string: &str) -> Result<Self, Box<dyn Error>> {
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

/// This structure represents a modular integer number.
/// This type implements the Field trait in order to provide
/// field behaviour for modular integers, but it is required
/// that users of this struct use actually prime numbers when
/// instantiating this type.
#[derive(Debug, Eq)]
pub struct ModInteger<'a> {
    value: Integer,
    prime: &'a Prime,
}

impl<'a> ModInteger<'a> {
    /// Creates a new modular integer modulus prime using the provided
    /// random number generator.
    pub fn random(prime: &'a Prime, rng: &mut Rng) -> Self {
        ModInteger {
            value: Integer::from(prime.value.random_below_ref(rng.internal_rep())),
            prime,
        }
    }

    /// Creates a new integer from the string.
    ///
    /// This method returns an error if an error occurs while parsing.
    pub fn parse(s: &str, prime: &'a Prime) -> Result<Self, ParseIntegerError> {
        Ok(ModInteger {
            value: Integer::from(Integer::parse(s)?).rem_euc(&prime.value),
            prime,
        })
    }
}

impl Field for ModInteger<'_> {
    /// Returns the modular integer 0 with the same
    /// modulus as the number that called this method.
    fn zero(&self) -> Self {
        ModInteger {
            value: Integer::new(),
            prime: self.prime,
        }
    }

    /// Returns the modular integer 1 with the same
    /// modulus as the number that called this method.
    fn one(&self) -> Self {
        ModInteger {
            value: Integer::from(1),
            prime: self.prime,
        }
    }

    /// Returns the additive inverse of this number.
    fn add_inverse(mut self) -> Self {
        self.value = &self.prime.value - self.value;
        self
    }

    /// Returns the multiplicative inverse of this number.
    ///
    /// # Panics
    ///
    /// This method panics if the modulus of this number is not a prime number.
    fn mul_inverse(mut self) -> Self {
        self.value = self
            .value
            .invert(&self.prime.value)
            .expect("Prime is not an actual prime");
        self
    }
}

// simple macro for panicking if two modular integers have different
// modulus.
macro_rules! panic_if_different_modulus {
    ($this:expr, $other:expr) => {
        if $this.prime != $other.prime {
            panic!("Illegal operation between different modulus numbers");
        }
    };
}

impl Add for ModInteger<'_> {
    type Output = Self;

    /// Returns the sum of two modular integers.
    ///
    /// It is required that this number and the other have
    /// the same prime as modulus.
    ///
    /// # Panics
    ///
    /// If the prime number of self is not the same as the prime
    /// number of rhs.
    fn add(self, rhs: Self) -> Self::Output {
        panic_if_different_modulus!(self, rhs);
        ModInteger {
            value: (self.value + rhs.value).rem_euc(&self.prime.value),
            ..self
        }
    }
}

impl Sub for ModInteger<'_> {
    type Output = Self;

    // TODO document this.
    fn sub(self, rhs: Self) -> Self::Output {
        panic_if_different_modulus!(self, rhs);
        ModInteger {
            value: (self.value - rhs.value).rem_euc(&self.prime.value),
            ..self
        }
    }
}

impl Mul for ModInteger<'_> {
    type Output = Self;

    // TODO document this.
    fn mul(self, rhs: Self) -> Self::Output {
        panic_if_different_modulus!(self, rhs);
        ModInteger {
            value: (self.value * rhs.value).rem_euc(&self.prime.value),
            ..self
        }
    }
}

impl Div for ModInteger<'_> {
    type Output = Self;

    // TODO document this.
    fn div(self, rhs: Self) -> Self::Output {
        panic_if_different_modulus!(self, rhs);
        if rhs.value == 0 {
            panic!("illegal division by 0");
        }
        ModInteger {
            value: (self.value
                * rhs
                    .value
                    .invert(&rhs.prime.value)
                    .expect("prime is not an actual prime"))
            .rem_euc(&self.prime.value),
            ..self
        }
    }
}

impl PartialEq for ModInteger<'_> {
    /// Returns true if both numbers are equal and
    /// have the same modulus.
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.prime == other.prime
    }
}

impl Display for ModInteger<'_> {
    /// Returns a string representation of this number.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

/////////////////////////////////
// Unit testing of the module. //
/////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::random::Rng;
    use rug::Integer;

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

    macro_rules! assert_valid_mod_int {
        ($number:expr, $prime:expr) => {
            assert!($number.value >= 0);
            assert!($number.value < $prime.value);
            assert_eq!(*$number.prime, $prime);
        };
    }

    #[test]
    fn mod_int_random() {
        let prime = Prime::parse("7").unwrap();
        let mut rng = Rng::new();
        for _ in 0..100 {
            let number = ModInteger::random(&prime, &mut rng);
            assert_valid_mod_int!(number, prime);
        }
    }

    macro_rules! parse_test {
        ($prime:expr, $num:expr, $mod_value:expr) => {
            let prime = Prime::parse(&$prime.to_string()).unwrap();
            let number = ModInteger::parse(&$num.to_string(), &prime).unwrap();
            assert_valid_mod_int!(number, prime);
            assert_eq!(number.value, Integer::from($mod_value));
        };
    }

    #[test]
    fn mod_int_parse_in_range() {
        parse_test!(11, 7, 7);
    }

    #[test]
    fn mod_int_parse_greater() {
        parse_test!(11, 100, 1);
    }

    #[test]
    fn mod_int_parse_negative() {
        parse_test!(7, -3, 4);
    }

    #[test]
    fn mod_int_zero() {
        // just call the method twice and check it returns the sam thing
        let prime = Prime::parse("3").unwrap();
        let base = ModInteger::parse("2", &prime).unwrap();
        let z1 = base.zero();
        let z2 = base.zero();
        assert_valid_mod_int!(z1, prime);
        assert_valid_mod_int!(z2, prime);
        assert_eq!(z1.value, z2.value);
    }

    #[test]
    fn mod_int_one() {
        // just call the method twice and check it returns the sam thing
        let prime = Prime::parse("3").unwrap();
        let base = ModInteger::parse("2", &prime).unwrap();
        let z1 = base.one();
        let z2 = base.one();
        assert_valid_mod_int!(z1, prime);
        assert_valid_mod_int!(z2, prime);
        assert_eq!(z1.value, z2.value);
    }

    #[test]
    fn mod_int_add_inverse() {
        let prime = Prime::parse("7").unwrap();
        let a = ModInteger::parse("5", &prime).unwrap();
        let inverse = a.add_inverse();
        assert_valid_mod_int!(inverse, prime);
        assert_eq!(inverse.value, Integer::from(2));
    }

    #[test]
    fn mod_int_mul_inverse() {
        let prime = Prime::parse("5").unwrap();
        let a = ModInteger::parse("3", &prime).unwrap();
        let inverse = a.mul_inverse();
        assert_valid_mod_int!(inverse, prime);
        assert_eq!(inverse.value, Integer::from(2));
    }

    macro_rules! operation_test {
        ($prime:expr, $lhs:expr, $rhs:expr, $op:tt, $expected:expr) => {
        let prime = Prime::parse($prime).unwrap();
        let lhs = ModInteger::parse($lhs, &prime).unwrap();
        let rhs = ModInteger::parse($rhs, &prime).unwrap();
        let result = lhs $op rhs;
        assert_valid_mod_int!(result, prime);
        assert_eq!(result.value, Integer::from($expected));
        }
    }

    #[test]
    fn mod_int_add_in_range() {
        operation_test!("11", "2", "5", +, 7);
    }

    #[test]
    fn mod_int_add_greater() {
        operation_test!("11", "10", "10", +, 9);
    }

    #[test]
    fn mod_int_add_identity() {
        operation_test!("13", "8", "0", +, 8);
    }

    #[test]
    fn mod_int_sub_in_range() {
        operation_test!("7", "6", "5", -, 1);
    }

    #[test]
    fn mod_int_sub_negative() {
        operation_test!("5", "1", "3", -, 3);
    }

    #[test]
    fn mod_int_mul_in_range() {
        operation_test!("13", "2", "3", *, 6);
    }

    #[test]
    fn mod_int_mul_greater() {
        operation_test!("11", "10", "9", *, 2);
    }

    #[test]
    fn mod_int_mul_identity() {
        operation_test!("23", "14", "1", *, 14);
    }

    #[test]
    fn mod_int_div_in_range() {
        operation_test!("7", "1", "6", /, 6);
    }

    #[test]
    fn mod_int_div_greater() {
        operation_test!("7", "2", "3", /, 3);
    }

    #[test]
    fn mod_int_div_identity() {
        operation_test!("11", "5", "1", /, 5);
    }

    #[test]
    fn mod_int_div_by_inverse() {
        operation_test!("7", "3", "5", /, 2);
    }

    #[test]
    fn mod_int_div_by_itself() {
        operation_test!("7", "5", "5", /, 1);
    }
}
