use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use rug::{
    integer::Order,
    ops::{RemRounding, RemRoundingAssign},
    Integer,
};

use crate::math::{error::ParseIntegerError, random::Rng, Field, Prime};

/// This structure represents a modular integer number.
/// This type implements the Field trait in order to provide
/// field behaviour for modular integers, but it is required
/// that users of this struct use actually prime numbers when
/// instantiating this type.
///
/// # Usage Example
/// ```
/// use shared_secrets::math::{Prime, ModInteger};
///
/// // Create a prime to use with the modular integers.
/// let prime = Prime::parse("7").unwrap();
///
/// // Create modular integers with the prime.
/// let a = ModInteger::parse("2", &prime).unwrap();
/// let mut b = ModInteger::parse("6", &prime).unwrap();
///
/// // Start having fun!
/// assert_eq!(ModInteger::parse("5", &prime).unwrap(), a / b);
///
/// ```
///
/// # Notes
///
/// In order to perform Operations between two modular integers, they must have
/// the same prime as modulus.
#[derive(Debug, Eq, Hash)]
pub struct ModInteger<'a> {
    value: Integer,
    prime: &'a Prime,
}

impl<'a> ModInteger<'a> {
    /// Creates a new modular integer modulus prime using the provided
    /// random number generator.
    ///
    /// # Parameters
    ///
    /// - prime: A prime from which to create a modular integer.
    /// - rng: A random number generator.
    ///
    /// # Returns
    ///
    /// A wrapped modular integer with the given prime as modulus.
    pub fn random(prime: &'a Prime, rng: &mut Rng) -> Self {
        ModInteger {
            value: Integer::from(prime.value.random_below_ref(rng.internal_rep())),
            prime,
        }
    }

    /// Creates a new integer from the string.
    ///
    /// # Parameters
    ///
    /// - s: A string from which to create a modular integer.
    /// - prime: A prime to have as modulus.
    ///
    /// # Returns
    ///
    /// A wrapped modular integer created from a string
    /// with the given prime as modulus.
    ///
    /// # Errors
    ///
    /// This method returns an error if an error occurs while parsing.
    pub fn parse(s: &str, prime: &'a Prime) -> Result<Self, ParseIntegerError> {
        Ok(ModInteger {
            value: Integer::from(Integer::parse(s)?).rem_euc(&prime.value),
            prime,
        })
    }

    /// Creates a new integer by parsing the string with the given radix
    ///
    /// # Errors
    /// This method returns an error if an error occurs while parsing.
    ///
    pub fn parse_radix(s: &str, prime: &'a Prime, radix: i32) -> Result<Self, ParseIntegerError> {
        Ok(ModInteger {
            value: Integer::from(Integer::parse_radix(s, radix)?).rem_euc(&prime.value),
            prime,
        })
    }

    /// Creates a new modular integer from a slice of bytes.
    ///
    /// # Parameters
    ///
    /// - digits: A slice of bytes from which to create
    /// a modular integer.
    /// - prime: A prime to have as modulus.
    ///
    /// # Returns
    ///
    /// A wrapped modular integer created from a slice of bytes
    /// with the given prime as modulus.
    ///
    /// # Notes
    ///
    /// This method assigns the most significant element first
    /// and treats each byte as little endian.
    pub fn from_digits(digits: &[u8], prime: &'a Prime) -> Self {
        ModInteger {
            value: Integer::from_digits(digits, Order::MsfLe).rem_euc(&prime.value),
            prime,
        }
    }

    /// Returns the zero of the finite field modulus the given prime.
    ///
    /// # Parameters
    ///
    /// - prime: A prime to have as modulus.
    ///
    /// # Returns
    ///
    /// The additive identity element of given field with the
    /// given prime as modulus.
    pub fn zero(prime: &'a Prime) -> Self {
        ModInteger {
            value: Integer::new(),
            prime,
        }
    }

    /// Returns the digits of this integer.
    ///
    /// This operation is guaranted to be the inverse
    /// operation of the from_digits integer creation
    /// as long as the number created with from_digits
    /// does not exceed prime.
    ///
    /// That is
    ///
    /// ```
    /// use shared_secrets::math::{Prime, ModInteger};
    ///
    /// let a: [u8; 3] = [1, 2, 3];
    /// let prime = Prime::parse("1000003").unwrap();
    /// let b = ModInteger::from_digits(&a, &prime).to_digits();
    /// for i in 0..3 {
    ///   assert_eq!(a[i], b[i]);
    /// }
    /// ```
    /// # Returns
    ///
    /// A vector of bytes with the digits of a modular integer.
    ///
    /// # Notes
    ///
    /// The digits are returned in Most significant digit first order
    /// and each byte is in litle endian.
    pub fn to_digits(&self) -> Vec<u8> {
        self.value.to_digits::<u8>(Order::MsfLe)
    }

    /// Returns a string representation of the number for the given radix.
    ///
    /// # Panics
    /// If the given radix is not in range 2 <= radix <= 36
    pub fn to_string_radix(&self, radix: i32) -> String {
        self.value.to_string_radix(radix)
    }
}

// gets the multiplicative inverse of an rug::Integer
macro_rules! invert {
    ($integer:ident) => {{
        $integer
            .value
            .invert(&$integer.prime.value)
            .expect("Prime is not an actual prime")
    }};
}

// gets the multiplicative inverse of an rug::Integer reference
macro_rules! invert_ref {
    ($integer:ident) => {{
        Integer::from(
            (&$integer.value)
                .invert_ref(&$integer.prime.value)
                .expect("Prime is not an actual prime"),
        )
    }};
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
        self.value = invert!(self);
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

macro_rules! base_operation {
    ($lhs:ident, $rhs:ident, $new_value:expr) => {{
        panic_if_different_modulus!($lhs, $rhs);
        ModInteger {
            value: ($new_value).rem_euc(&$lhs.prime.value),
            prime: $lhs.prime,
        }
    };};
}

macro_rules! operation {
    ($trait:ident, $method:ident, $op:tt) => {


impl $trait for ModInteger<'_> {
    type Output = Self;

    fn $method(self, rhs: Self) -> Self::Output {
        base_operation!(self, rhs, self.value $op rhs.value)
    }
}

impl<'a> $trait<ModInteger<'_>> for &ModInteger<'a> {
    type Output = ModInteger<'a>;

    fn $method(self, rhs: ModInteger<'_>) -> Self::Output {
        base_operation!(self, rhs, &self.value $op rhs.value)
    }
}

impl<'a> $trait<&ModInteger<'_>> for ModInteger<'a> {
    type Output = ModInteger<'a>;

    fn $method(self, rhs: &ModInteger<'_>) -> Self::Output {
        base_operation!(self, rhs, self.value $op &rhs.value)
    }
}

impl<'a> $trait<&ModInteger<'_>> for &ModInteger<'a> {
    type Output = ModInteger<'a>;

    fn $method(self, rhs: &ModInteger<'_>) -> Self::Output {
        base_operation!(self, rhs, Integer::from(&self.value $op &rhs.value))
    }
}

}
}

operation!(Add, add, +);
operation!(Sub, sub, -);
operation!(Mul, mul, *);

impl Div for ModInteger<'_> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.value == 0 {
            panic!("illegal division by 0");
        }
        base_operation!(self, rhs, self.value * invert!(rhs))
    }
}

impl<'a> Div<ModInteger<'_>> for &ModInteger<'a> {
    type Output = ModInteger<'a>;

    fn div(self, rhs: ModInteger<'_>) -> Self::Output {
        if rhs.value == 0 {
            panic!("illegal division by 0");
        }
        base_operation!(self, rhs, &self.value * invert!(rhs))
    }
}

impl<'a> Div<&ModInteger<'_>> for ModInteger<'a> {
    type Output = ModInteger<'a>;

    fn div(self, rhs: &ModInteger<'_>) -> Self::Output {
        if rhs.value == 0 {
            panic!("illegal division by 0");
        }
        base_operation!(self, rhs, self.value * invert_ref!(rhs))
    }
}

impl<'a> Div<&ModInteger<'_>> for &ModInteger<'a> {
    type Output = ModInteger<'a>;

    fn div(self, rhs: &ModInteger<'_>) -> Self::Output {
        if rhs.value == 0 {
            panic!("illegal division by 0");
        }
        base_operation!(self, rhs, &self.value * invert_ref!(rhs))
    }
}

macro_rules! assign_op {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait for ModInteger<'_> {

            fn $method(&mut self, rhs: Self) {
                panic_if_different_modulus!(self, rhs);
                self.value $op rhs.value;
                self.value.rem_euc_assign(&self.prime.value);
            }
        }

        impl $trait<&ModInteger<'_>> for ModInteger<'_> {

            fn $method(&mut self, rhs: &ModInteger<'_>) {
                panic_if_different_modulus!(self, rhs);
                self.value $op &rhs.value;
                self.value.rem_euc_assign(&self.prime.value);
            }
        }
    }
}

assign_op!(AddAssign, add_assign, +=);
assign_op!(SubAssign, sub_assign, -=);
assign_op!(MulAssign, mul_assign, *=);

impl DivAssign for ModInteger<'_> {
    fn div_assign(&mut self, rhs: Self) {
        panic_if_different_modulus!(self, rhs);
        if rhs.value == 0 {
            panic!("illegal division by 0");
        }
        self.value *= rhs
            .value
            .invert(&rhs.prime.value)
            .expect("prime is not an actual prime");
        self.value.rem_euc_assign(&self.prime.value);
    }
}

impl DivAssign<&ModInteger<'_>> for ModInteger<'_> {
    fn div_assign(&mut self, rhs: &ModInteger<'_>) {
        panic_if_different_modulus!(self, rhs);
        if rhs.value == 0 {
            panic!("illegal division by 0");
        }
        self.value *= Integer::from(
            (&rhs.value)
                .invert_ref(&rhs.prime.value)
                .expect("prime is not an actual prime"),
        );
        self.value.rem_euc_assign(&self.prime.value);
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
    use rug::Integer;

    use crate::math::random::Rng;

    use super::*;

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

    macro_rules! test_from_digits {
        ($digits:expr, $prime:expr, $expected:expr) => {
            let prime = Prime::parse($prime).unwrap();
            let result = ModInteger::from_digits(&$digits, &prime);
            assert_valid_mod_int!(result, prime);
            assert_eq!(result.value, $expected);
        };
    }

    #[test]
    fn mod_int_from_digits_general() {
        test_from_digits!([0x1, 0x14, 0x6], "1000003", 0x11406);
    }

    #[test]
    fn mod_int_from_digits_single_digits() {
        test_from_digits!([0x1, 0x2, 0x3], "1000003", 0x010203);
    }

    #[test]
    fn mod_int_from_digits_example() {
        test_from_digits!([0x12, 0x34, 0x56, 0x78], "5915587277", 0x1234_5678);
    }

    #[test]
    fn mod_int_new_zero() {
        let prime = Prime::parse("7").unwrap();
        let result = ModInteger::zero(&prime);
        assert_eq!(result.value, 0);
        assert_eq!(*result.prime, prime);
    }

    macro_rules! test_to_digits {
        ($number:expr, $prime:expr, $expected:expr) => {
            let prime = Prime::parse($prime).unwrap();
            let result = ModInteger::parse($number, &prime).unwrap().to_digits();
            assert_eq!(result, $expected);
        };
    }

    #[test]
    fn mod_int_to_digits() {
        test_to_digits!("66051", "1000003", [0x1, 0x2, 0x3]);
    }

    #[test]
    fn mod_int_digits() {
        // checks that to_digits is the inverse function of
        // from_digits
        let digits = [0xf, 0x12, 0xe];
        let prime = Prime::parse("5915587277").unwrap();
        let result = ModInteger::from_digits(&digits, &prime).to_digits();
        for i in 0..3 {
            assert_eq!(digits[i], result[i]);
        }
    }

    #[test]
    fn mod_int_digits_reverse() {
        // The same as before but in the reverse order.
        let prime = Prime::parse("1000003").unwrap();
        let mut rng = Rng::new();
        let number = ModInteger::random(&prime, &mut rng);
        let result = ModInteger::from_digits(&number.to_digits(), &prime);
        assert_eq!(result.value, number.value);
    }

    #[test]
    fn mod_int_zero() {
        // just call the method twice and check it returns the same thing
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
        // just call the method twice and check it returns the same thing
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

    macro_rules! ref_val_operation_test {
        ($prime:expr, $lhs:expr, $rhs:expr, $op:tt, $expected:expr) => {
        let prime = Prime::parse($prime).unwrap();
        let lhs = ModInteger::parse($lhs, &prime).unwrap();
        let rhs = ModInteger::parse($rhs, &prime).unwrap();
        let result = &lhs $op rhs;
        assert_valid_mod_int!(result, prime);
        assert_eq!(result.value, Integer::from($expected));
        }
    }

    macro_rules! val_ref_operation_test {
        ($prime:expr, $lhs:expr, $rhs:expr, $op:tt, $expected:expr) => {
        let prime = Prime::parse($prime).unwrap();
        let lhs = ModInteger::parse($lhs, &prime).unwrap();
        let rhs = ModInteger::parse($rhs, &prime).unwrap();
        let result = lhs $op &rhs;
        assert_valid_mod_int!(result, prime);
        assert_eq!(result.value, Integer::from($expected));
        }
    }

    macro_rules! refs_operation_test {
        ($prime:expr, $lhs:expr, $rhs:expr, $op:tt, $expected:expr) => {
        let prime = Prime::parse($prime).unwrap();
        let lhs = ModInteger::parse($lhs, &prime).unwrap();
        let rhs = ModInteger::parse($rhs, &prime).unwrap();
        let result = &lhs $op &rhs;
        assert_valid_mod_int!(result, prime);
        assert_eq!(result.value, Integer::from($expected));
        }
    }

    macro_rules! assign_operation_test {
        ($prime:expr, $lhs:expr, $rhs:expr, $op:tt, $expected:expr) => {
        let prime = Prime::parse($prime).unwrap();
        let mut lhs = ModInteger::parse($lhs, &prime).unwrap();
        let rhs = ModInteger::parse($rhs, &prime).unwrap();
        lhs $op rhs;
        assert_valid_mod_int!(lhs, prime);
        assert_eq!(lhs.value, Integer::from($expected));
        }
    }

    macro_rules! ref_assign_operation_test {
        ($prime:expr, $lhs:expr, $rhs:expr, $op:tt, $expected:expr) => {
        let prime = Prime::parse($prime).unwrap();
        let mut lhs = ModInteger::parse($lhs, &prime).unwrap();
        let rhs = ModInteger::parse($rhs, &prime).unwrap();
        lhs $op &rhs;
        assert_valid_mod_int!(lhs, prime);
        assert_eq!(lhs.value, Integer::from($expected));
        }
    }

    macro_rules! all_ops_test {
        ($prime:expr, $lhs:expr, $rhs:expr, $op:tt, $expected:expr) => {
            operation_test!($prime, $lhs, $rhs, $op, $expected);
            val_ref_operation_test!($prime, $lhs, $rhs, $op, $expected);
            ref_val_operation_test!($prime, $lhs, $rhs, $op, $expected);
            refs_operation_test!($prime, $lhs, $rhs, $op, $expected);
        };
    }

    macro_rules! all_assign_ops_test {
        ($prime:expr, $lhs:expr, $rhs:expr, $op:tt, $expected:expr) => {
            assign_operation_test!($prime, $lhs, $rhs, $op, $expected);
            ref_assign_operation_test!($prime, $lhs, $rhs, $op, $expected);
        };
    }

    #[test]
    fn mod_int_add_in_range() {
        all_ops_test!("11", "2", "5", +, 7);
        all_assign_ops_test!("11", "2", "5", +=, 7);
    }

    #[test]
    fn mod_int_add_greater() {
        all_ops_test!("11", "10", "10", +, 9);
        all_assign_ops_test!("11", "10", "10", +=, 9);
    }

    #[test]
    fn mod_int_add_identity() {
        all_ops_test!("13", "8", "0", +, 8);
        all_assign_ops_test!("13", "8", "0", +=, 8);
    }

    #[test]
    fn mod_int_sub_in_range() {
        all_ops_test!("7", "6", "5", -, 1);
        all_assign_ops_test!("7", "6", "5", -=, 1);
    }

    #[test]
    fn mod_int_sub_negative() {
        all_ops_test!("5", "1", "3", -, 3);
        all_assign_ops_test!("5", "1", "3", -=, 3);
    }

    #[test]
    fn mod_int_mul_in_range() {
        all_ops_test!("13", "2", "3", *, 6);
        all_assign_ops_test!("13", "2", "3", *=, 6);
    }

    #[test]
    fn mod_int_mul_greater() {
        all_ops_test!("11", "10", "9", *, 2);
        all_assign_ops_test!("11", "10", "9", *=, 2);
    }

    #[test]
    fn mod_int_mul_identity() {
        all_ops_test!("23", "14", "1", *, 14);
        all_assign_ops_test!("23", "14", "1", *=, 14);
    }

    #[test]
    fn mod_int_div_in_range() {
        all_ops_test!("7", "1", "6", /, 6);
        all_assign_ops_test!("7", "1", "6", /=, 6);
    }

    #[test]
    fn mod_int_div_greater() {
        all_ops_test!("7", "2", "3", /, 3);
        all_assign_ops_test!("7", "2", "3", /=, 3);
    }

    #[test]
    fn mod_int_div_identity() {
        all_ops_test!("11", "5", "1", /, 5);
        all_assign_ops_test!("11", "5", "1", /=, 5);
    }

    #[test]
    fn mod_int_div_by_inverse() {
        all_ops_test!("7", "3", "5", /, 2);
        all_assign_ops_test!("7", "3", "5", /=, 2);
    }

    #[test]
    fn mod_int_div_by_itself() {
        all_ops_test!("7", "5", "5", /, 1);
        all_assign_ops_test!("7", "5", "5", /=, 1);
    }
}
