use crate::math::*;

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
        Polynomial::Coefficients(CoeffPolynomial {
            coefficients: coeffs,
        })
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
        let y = self
            .coefficients
            .iter()
            .rev()
            .fold(self.coefficients[0].zero(), |acc, coeff| acc * &x + coeff);
        (x, y)
    }
}

impl<'a> InterpolationPolynomial<'a> {
    pub fn eval(&self, x: ModInteger<'a>) -> Evaluation<'a> {
        let mut result = self.evaluations[0].0.zero();
        for (i, (_, y)) in self.evaluations.iter().enumerate() {
            result += y * self.eval_base_polynomial(&x, i);
        }
        (x, result)
    }

    fn eval_base_polynomial(&self, x: &ModInteger<'a>, i: usize) -> ModInteger<'a> {
        let (x_i, _) = &self.evaluations[i];
        let mut result = x_i.one();
        for x_j in self
            .evaluations
            .iter()
            .enumerate()
            .filter(|(k, _)| *k != i)
            .map(|(_, (v, _))| v)
        {
            result *= (x - x_j) / (x_i - x_j);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{Field, ModInteger, Prime};

    #[test]
    fn test_horner() {
        let prime = Prime::parse("648863").unwrap();
        let x = ModInteger::parse("3", &prime).unwrap();
        let one = x.one();
        let polynomial = Polynomial::from_coef(vec![x.one(), x.one(), x]);
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
        let poly = Polynomial::from_evals(vec![(x_1, y_1), (x_2, y_2), (x_3, y_3)]);
        let (x, y) = poly.eval(zero);

        assert_eq!(x, ModInteger::zero(&prime));
        assert_eq!(y, ModInteger::parse("1", &prime).unwrap());
    }
}
