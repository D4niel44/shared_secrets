use crate::math::{field::Field, modular::*};
use std::ops::{Add, Mul};

pub type Evaluation<'a> = (ModInteger<'a>, ModInteger<'a>);

pub struct CoeffPolynomial<'a> {
    coefficients: Vec<ModInteger<'a>>,
}

pub struct InterpolationPolynomial<'a> {
    evaluations: Vec<Evaluation<'a>>,
}

pub enum Polynomial<'a> {
    Coefficients(CoeffPolynomial<'a>),
    Interpolation(InterpolationPolynomial<'a>),
}

impl<'a> Polynomial<'a> {
    pub fn from_coef(coeffs: Vec<ModInteger<'a>>) -> Self {
        Polynomial::Coefficients(CoeffPolynomial { coefficients: coeffs })
    }

    pub fn from_evals(evals: Vec<Evaluation<'a>>) -> Self {
        Polynomial::Interpolation(InterpolationPolynomial { evaluations: evals })
    }

    pub fn eval(&self, x: ModInteger<'a>) -> Evaluation<'a> {
        match self {
            Polynomial::Coefficients(poly) => poly.eval(x),
            Polynomial::Interpolation(poly) => poly.eval(x),
        }
    }
}

impl<'a> CoeffPolynomial<'a> {
    pub fn eval(&self, x: ModInteger<'a>) -> Evaluation<'a> {
        let y = self.coefficients
            .iter()
            .rev()
            .fold(self.coefficients[0].zero(), |acc, coeff| acc * &x + coeff);
        (x, y)
    }
}

impl<'a> InterpolationPolynomial<'a> {
    pub fn eval(&self, x: ModInteger<'a>) -> Evaluation<'a> {
        //TODO lagrange
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{ModInteger, Prime, Field};

    #[test]
    fn test_horner() {
        let p = Prime::parse("648863").unwrap();
        let x = ModInteger::parse("3", &p).unwrap();
        let one = x.one();
        let polynomial = Polynomial::from_coef(vec![x.one(), x.one(), x]);
        let (_, result) = polynomial.eval(one);

        assert_eq!(result, ModInteger::parse("5", &p).unwrap());
    }
}

