use std::collections::{hash_set, HashSet};
use std::error::Error;

use crate::math::{error::ParseIntegerError, random::Rng};
use crate::math::{Evaluation, ModInteger, Polynomial, Prime};

const PRIME_257: &str =
    "208351617316091241234326746312124448251235562226470491514186331217050270460481";

// Radix used when transforming shares to evaluations
const RADIX: i32 = 36;

/// A share of the secret.
pub type Share = (String, String);

/// A consuming Iterator over shares.
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
/// A ShareIter with n Shares.
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
    let zero = ModInteger::zero(&prime);
    let mut rng = Rng::new();
    let secret_number = ModInteger::from_digits(secret, &prime);

    // Create the polynomial
    let mut coefficients = Vec::with_capacity(k);
    coefficients.push(secret_number);
    if k > 1 {
        for _ in 1..k - 2 {
            coefficients.push(ModInteger::random(&prime, &mut rng));
        }
        // Ensure last element is not zero
        coefficients.push(non_zero_random(&prime, &mut rng, &zero));
    }
    let polynomial = Polynomial::from_coefficients(coefficients);

    // Compute n random evaluations of polynomial
    let mut evaluations = HashSet::with_capacity(n);
    while evaluations.len() < n {
        let x = non_zero_random(&prime, &mut rng, &zero);
        let eval = polynomial.eval(x);
        evaluations.insert((eval.0.to_string_radix(RADIX), eval.1.to_string_radix(RADIX)));
    }

    evaluations.into_iter()
}

// Computes a random ModInteger that does not equals zero.
fn non_zero_random<'a>(prime: &'a Prime, rng: &mut Rng, zero: &ModInteger) -> ModInteger<'a> {
    let mut random = ModInteger::random(&prime, rng);
    while random == *zero {
        random = ModInteger::random(&prime, rng);
    }
    random
}

/// Recovers a secret given an Iterator of Shares.
///
/// # Parameters
///
/// - shares: An Iterator of shares, require that each share has a
/// unique first element.
///
/// # Returns
///
/// A vector of bytes containing the original secret.
///
/// # Errors
///
/// This method returns an error if it cannot parse the integers
/// or if there are two shares with the same first element.
pub fn recover_secret(shares: impl Iterator<Item = Share>) -> Result<Vec<u8>, Box<dyn Error>> {
    let prime = Prime::parse(PRIME_257).unwrap();
    let evaluations = shares
        .map::<Result<Evaluation, ParseIntegerError>, _>(|(x, y)| {
            Ok((
                ModInteger::parse_radix(&x, &prime, RADIX)?,
                ModInteger::parse_radix(&y, &prime, RADIX)?,
            ))
        })
        .collect::<Result<Vec<Evaluation>, _>>()?;
    let polynomial = Polynomial::from_evals(evaluations)?;
    let (_, secret_number) = polynomial.eval(ModInteger::zero(&prime));
    Ok(secret_number.to_digits())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! split_secret_test {
        ($secret:expr, $n:expr, $k:expr, $take:expr) => {
            let secret = $secret;
            let result = split_secret(&secret, $n, $k);
            assert_eq!(result.len(), $n);
            let returned_secret = recover_secret(result.take($take)).unwrap();
            assert_eq!(secret, returned_secret);
        };
    }

    #[test]
    fn secret_empty_vec() {
        split_secret_test!(vec![], 5, 3, 3);
    }

    #[test]
    fn secret_zeroes_vec() {
        split_secret_test!(vec![], 4, 3, 3);
    }

    #[test]
    fn secret_minimum_n() {
        split_secret_test!(vec![0x12u8, 0x87u8], 3, 1, 1);
    }

    #[test]
    fn secret_minimum_k() {
        split_secret_test!(vec![0x67, 0xa, 0xc, 0xe], 10, 1, 1);
    }

    #[test]
    fn secret_n_equals_k() {
        split_secret_test!(vec![0x1u8, 0x2u8, 0x3u8, 0x35u8], 5, 5, 5);
    }

    #[test]
    fn secret_integrity() {
        split_secret_test!(vec![0x11u8, 0xffu8, 0x35u8, 0x4eu8], 4, 2, 2);
    }

    #[test]
    fn secret_more_than_k_evals() {
        split_secret_test!(vec![0xafu8, 0xbbu8, 0x13u8, 0x01u8], 10, 3, 9);
    }
}
