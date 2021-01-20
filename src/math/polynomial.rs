use std::collections::HashMap;

use crate::math::error::ValueError;
use crate::math::*;

/// A modular Evaluation of a polynomial
pub type Evaluation<'a> = (ModInteger<'a>, ModInteger<'a>);

/// A Modular Integers coefficients polynomial
pub struct CoeffPolynomial<'a> {
    coefficients: Vec<ModInteger<'a>>,
}

/// A Modular Integer interpolation polynomial
pub struct InterpolationPolynomial<'a> {
    evaluations: Vec<Evaluation<'a>>,
}

/// A polynomial, either created from coefficients
/// or from interpolation.
pub enum Polynomial<'a> {
    Coefficients(CoeffPolynomial<'a>),
    Interpolation(InterpolationPolynomial<'a>),
}

impl<'a> Polynomial<'a> {
    /// Creates a new polynomial given the vector of coefficients.
    ///
    /// The x[i] element of the vector is treated as the coefficient
    /// for the term x^i
    ///
    /// # Parameters
    ///
    /// - coefficients: A vector of coefficients. It is required that this
    /// vector is not empty and that the last coefficient is not zero.
    ///
    /// # Returns
    ///
    /// A new polynomial with the given coefficients.
    ///
    /// # Panics
    ///
    /// This method panics if the constraints above are not met.
    pub fn from_coefficients(coefficients: Vec<ModInteger<'a>>) -> Self {
        Polynomial::Coefficients(CoeffPolynomial::new(coefficients))
    }

    /// Creates a new polynomial given a vector of evaluations.
    ///
    /// # Parameters
    ///
    /// - evaluations: A vector of Evaluations. Each evaluation should have a unique
    /// first value and the evaluations vector should not be empty.
    ///
    /// # Returns
    ///
    /// A polynomial that satisfies the given evaluations.
    ///
    /// # Errors
    ///
    /// A ValueError if the constrains are not met.
    pub fn from_evals(evaluations: Vec<Evaluation<'a>>) -> Result<Self, ValueError> {
        Ok(Polynomial::Interpolation(InterpolationPolynomial::new(
            evaluations,
        )?))
    }

    /// Returns the result from evaluating this polynomial.
    ///
    /// # Parameters
    ///
    /// - int: Modular integer from wich the evaluation is made.
    ///
    /// # Returns
    ///
    /// An evaluation of the given polynomial.
    pub fn eval(&self, int: ModInteger<'a>) -> Evaluation<'a> {
        match self {
            Polynomial::Coefficients(poly) => poly.eval(int),
            Polynomial::Interpolation(poly) => poly.eval(int),
        }
    }
}

impl<'a> CoeffPolynomial<'a> {
    /// Same as Polynomial::from_coefficients, but returns a CoeffPolynomial instead.
    ///
    /// # Parameters
    ///
    /// - coefficients: A vector of coefficients. It is required that this
    /// vector is not empty and that the last coefficient is not zero.
    ///
    /// # Returns
    ///
    /// A CoeffPolynomial with the given coefficients.
    ///
    /// # Notes
    ///
    /// It is encouraged to create this type of polynomial using Polynomial::from_coefficients
    /// instead of using this method.
    pub fn new(coefficients: Vec<ModInteger<'a>>) -> Self {
        if coefficients.len() == 0 {
            panic!("Tried to create polynomial with zero elements");
        }
        if *coefficients.last().unwrap() == coefficients[0].zero() {
            panic!("Last coefficient is zero");
        }
        CoeffPolynomial { coefficients }
    }

    /// Returns the result from evaluating this polynomial
    /// by use of the horner method.
    ///
    /// # Parameters
    ///
    /// - x: Modular integer representing a coefficient.
    ///
    /// # Returns
    ///
    /// An evaluation of the given polynomial.
    ///
    /// # Notes
    ///
    /// Discouraged in favor of Polynomial.eval()
    pub fn eval(&self, x: ModInteger<'a>) -> Evaluation<'a> {
        let y = self
            .coefficients
            .iter()
            .rev()
            .fold(self.coefficients[0].zero(), |acc, a_i| acc * &x + a_i);
        (x, y)
    }
}

impl<'a> InterpolationPolynomial<'a> {
    /// Same as Polynomial::from_evals, but returns a InterpolationPolynomial instead.
    ///
    /// # Parameters
    ///
    /// - evaluations: A vector of Evaluations. Each evaluation should have a unique
    /// first value and the evaluations vector should not be empty.
    ///
    /// # Returns
    ///
    /// An InterpolationPolynomial from the given evaluations.
    ///
    /// # Notes
    ///
    /// It is encouraged to create this type of polynomial using Polynomial::from_evals
    /// instead of using this method.
    pub fn new(evaluations: Vec<Evaluation<'a>>) -> Result<Self, ValueError> {
        if evaluations.len() == 0 {
            return Err(ValueError("No evaluations were provided".into()));
        }
        let mut unique_evals = HashMap::with_capacity(evaluations.len());
        for (x, y) in &evaluations {
            if unique_evals.contains_key(x) {
                return Err(ValueError("Duplicated x across Evaluation".into()));
            }
            unique_evals.insert(x, y);
        }
        Ok(InterpolationPolynomial { evaluations })
    }

    /// Returns the result from evaluating this polynomial
    /// by use of lagrange interpolation.
    ///
    /// # Parameters
    ///
    /// - x: Modular integer representing the result of
    /// an evaluation.
    ///
    /// # Returns
    ///
    /// An evaluation from the given polynomial.
    ///
    /// # Notes
    ///
    /// Discouraged in favor of Polynomial.eval()
    pub fn eval(&self, x: ModInteger<'a>) -> Evaluation<'a> {
        let y = self
            .evaluations
            .iter()
            .enumerate()
            .fold(self.evaluations[0].0.zero(), |acc, (i, (_, y))| {
                acc + (y * self.eval_base_polynomial(&x, i))
            });
        (x, y)
    }

    // evaluates the ith Lagrange base polynomial
    fn eval_base_polynomial(&self, x: &ModInteger<'a>, i: usize) -> ModInteger<'a> {
        let (x_i, _) = &self.evaluations[i];
        self.evaluations
            .iter()
            .enumerate()
            .filter(|(j, _)| i != *j)
            .map(|(_, (v, _))| v)
            .fold(x_i.one(), |acc, x_j| acc * (x - x_j) / (x_i - x_j))
    }
}

/////////////////////////////////
// Unit testing of the module. //
/////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{Field, ModInteger, Prime};

    #[test]
    fn test_horner() {
        let prime = Prime::parse("648863").unwrap();
        let x = ModInteger::parse("3", &prime).unwrap();
        let one = x.one();
        let polynomial = Polynomial::from_coefficients(vec![x.one(), x.one(), x]);
        let (_, result) = polynomial.eval(one);

        assert_eq!(result, ModInteger::parse("5", &prime).unwrap());
    }

    #[test]
    fn test_lagrange() {
        let prime = Prime::parse("7").unwrap();
        let x_1 = ModInteger::parse("1", &prime).unwrap();
        let x_2 = ModInteger::parse("2", &prime).unwrap();
        let x_3 = ModInteger::parse("3", &prime).unwrap();
        let y_1 = ModInteger::parse("3", &prime).unwrap();
        let y_2 = ModInteger::parse("0", &prime).unwrap();
        let y_3 = ModInteger::parse("6", &prime).unwrap();
        let zero = x_1.zero();
        let poly = Polynomial::from_evals(vec![(x_1, y_1), (x_2, y_2), (x_3, y_3)]).unwrap();
        let (x, y) = poly.eval(zero);

        assert_eq!(x, ModInteger::zero(&prime));
        assert_eq!(y, ModInteger::parse("1", &prime).unwrap());
    }
}
