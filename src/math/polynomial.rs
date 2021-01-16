use crate::math::field::Field;

type Evaluation<T: Field> = (T, T);

struct CoeffPolynomial<T: Field> {
    coefficients: Vec<T>,
}

struct InterpolationPolynomial<T: Field> {
    evaluations: Vec<Evaluation<T>>,
}

enum Polynomial<T: Field> {
    Coefficients(CoeffPolynomial<T>),
    Interpolation(InterpolationPolynomial<T>),
}

impl<T: Field> Polynomial<T> {
    fn from_coef(arr: Vec<T>) -> Self {
        Polynomial::Coefficients(CoeffPolynomial { coefficients: arr })
    }

    fn from_evals(arr: Vec<Evaluation<T>>) -> Self {
        Polynomial::Interpolation(InterpolationPolynomial { evaluations: arr })
    }

    fn eval(&self, x: T) -> Evaluation<T> {
        //TODO match name {}
        unimplemented!()
    }
}

impl<T: Field> CoeffPolynomial<T> {
    fn eval(&self, x: T) -> Evaluation<T> {
        //TODO
        unimplemented!()
    }
}

impl<T: Field> InterpolationPolynomial<T> {
    fn eval(&self, x: T) -> Evaluation<T> {
        //TODO
        unimplemented!()
    }
}
