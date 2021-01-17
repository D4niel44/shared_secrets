use std::collections::{hash_set, HashSet};

use crate::math::random::Rng;
use crate::math::{ModInteger, Polynomial, Prime};

const PRIME_257: &str =
    "208351617316091241234326746312124448251235562226470491514186331217050270460481";

/// A share of the secret.
pub type Share = (String, String);

pub type ShareIter = hash_set::IntoIter<Share>;

/// Splits a secret using shamir secret sharing scheme.
///
/// # Parameters
///
/// - secret: The secret to share.
/// - n: The total number of shares to return (n > 2).
/// - k: The minimum number of shares to decipher the secret (0 < k <= n).
///
/// # Returns
/// A vector with n evaluations of teh polynomial.
///
/// # Panics
///
/// This method panics if the parameters constraints are not met.
pub fn split_secret(secret: &[u8], n: usize, k: usize) -> ShareIter {
    if n <= 2 {
        panic!("n must be greater than 2");
    }
    if k == 0 || k > n {
        panic!("k must be in the range 0 < k <= n");
    }

    // Initialize values
    let prime = Prime::parse(PRIME_257).unwrap();
    let mut rng = Rng::new();
    let secret_number = ModInteger::from_digits(secret, &prime);

    // Create the polynomial
    let mut coefficients = Vec::with_capacity(k);
    coefficients.push(secret_number);
    for _ in 1..k - 1 {
        coefficients.push(ModInteger::random(&prime, &mut rng));
    }
    // Ensure last element is not zero
    let mut last_coefficient = ModInteger::random(&prime, &mut rng);
    let zero = ModInteger::parse("0", &prime).unwrap();
    while last_coefficient == zero {
        last_coefficient = ModInteger::random(&prime, &mut rng);
    }
    let polynomial = Polynomial::from_coef(coefficients);

    // Compute n random evaluations of polynomial
    let mut evaluations = HashSet::with_capacity(n);
    while evaluations.len() < n {
        let x = ModInteger::random(&prime, &mut rng);
        let eval = polynomial.eval(x);
        evaluations.insert((eval.0.to_string(), eval.1.to_string()));
    }

    evaluations.into_iter()
}

pub fn recover_secret(evals: impl Iterator<Item = Share>) -> Vec<u8> {
    let prime = Prime::parse(PRIME_257).unwrap();
    let polynomial = Polynomial::from_evals(
        evals
            .map(|(x, y)| {
                (
                    ModInteger::parse(&x, &prime).unwrap(),
                    ModInteger::parse(&y, &prime).unwrap(),
                )
            })
            .collect(),
    );
    let (_, secret_number) = polynomial.eval(ModInteger::parse("0", &prime).unwrap());
    secret_number.to_digits()
}

#[cfg(test)]
mod tests {

    use super::*;

    macro_rules! split_secret_test {
        ($secret:expr, $n:expr, $k:expr) => {
            let secret = $secret;
            let result = split_secret(&secret, $n, $k);
            assert_eq!(result.len(), $n);
            let returned_secret = recover_secret(result.take($k));
            assert_eq!(secret, returned_secret);
        };
    }

    #[test]
    fn secret_empty_vec() {
        split_secret_test!(vec![], 5, 3);
    }

    #[test]
    fn secret_zeroes_vec() {
        split_secret_test!(vec![0x0u8, 0x0u8, 0x0u8], 4, 3);
    }

    #[test]
    fn secret_minimum_n() {
        split_secret_test!(vec![0x12u8, 0x87u8], 2, 1);
    }

    #[test]
    fn secret_minimum_k() {
        split_secret_test!(vec![0x67, 0xa, 0xc, 0xe], 10, 1);
    }

    #[test]
    fn secret_n_equals_k() {
        split_secret_test!(vec![0x1u8, 0x2u8, 0x3u8, 0x35u8], 5, 5);
    }

    // split a secret and recover it.
    #[test]
    fn secret_integrity() {
        split_secret_test!(vec![0x11u8, 0xffu8, 0x35u8, 0x4eu8], 4, 2);
    }
}
