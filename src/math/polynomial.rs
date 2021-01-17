use crate::math::field::Field;

pub type Evaluation<T: Field> = (T, T);

pub struct CoeffPolynomial<T: Field> {
    coefficients: Vec<T>,
}

pub struct InterpolationPolynomial<T: Field> {
    evaluations: Vec<Evaluation<T>>,
}

pub enum Polynomial<T: Field> {
    Coefficients(CoeffPolynomial<T>),
    Interpolation(InterpolationPolynomial<T>),
}

impl<T: Field> Polynomial<T> {
    pub fn from_coef(arr: Vec<T>) -> Self {
        Polynomial::Coefficients(CoeffPolynomial { coefficients: arr })
    }

    pub fn from_evals(arr: Vec<Evaluation<T>>) -> Self {
        Polynomial::Interpolation(InterpolationPolynomial { evaluations: arr })
    }

    pub fn eval(&self, x: T) -> Evaluation<T> {
        //TODO match name {}
        unimplemented!()
    }
}

impl<T: Field> CoeffPolynomial<T> {
    pub fn eval(&self, x: T) -> Evaluation<T> {
        //TODO
        unimplemented!()
    }
}

impl<T: Field> InterpolationPolynomial<T> {
    pub fn eval(&self, x: T) -> Evaluation<T> {
        //TODO
        unimplemented!()
    }
}
