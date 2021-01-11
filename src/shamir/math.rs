use std::ops::{Add, Div, Mul, Sub};

/// Represents a mathemathical field.
/// Provides methods for adding, multiplying
/// and getting identities and inverses.
///
/// Types implementing this trait are not only require to
/// implement traits Add, Sub, Mul, Div but they also require
/// Add to behave as the field add operation, Mul to behave as
/// the field dot operation, Sub to be the addition by the additive
/// inverse (a - b must equal a + b.add_inverse()) and Div to be the
/// multiplication by the multiplicative inverse (a / b must equal
/// a * b.mul_inverse() if b != 0)
///
pub trait Field: Sized + Add + Sub + Mul + Div + PartialEq {
    /// Returns the additive identity of this field.
    /// Implementations of this method should always return the
    /// same result.
    fn zero() -> Self;

    /// Returns the multiplicative identity of this field.
    /// Implementations of this method should always return the
    /// same result.
    fn one() -> Self;

    /// Returns the additive inverse of this number.
    fn add_inverse(&self) -> Self;

    /// Returns the multiplicative inverse of this number.
    ///
    /// # Panics
    ///
    /// Panics if called on the zero of the field.
    fn mul_inverse(&self) -> Self;

    /// Returns true if this element is the zero of the field.
    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }

    /// returns true if this is the one of the field.
    fn is_one(&self) -> bool {
        *self == Self::one()
    }
}
