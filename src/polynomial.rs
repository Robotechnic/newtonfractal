use num_complex::Complex;
use std::{
    fmt::Debug,
    ops::{Mul, MulAssign},
};

#[derive(Clone)]
pub struct Polynomial {
    coefficients: Vec<Complex<f64>>, // coefficients of the polynomial in the form a_n * x^n + ... + a_1 * x + a_0
}

impl Default for Polynomial {
    fn default() -> Self {
        Self {
            coefficients: vec![Complex::new(1.0, 0.0)],
        }
    }
}

impl Polynomial {
    pub fn new(coeff: Vec<Complex<f64>>) -> Self {
        if coeff.is_empty() {
            panic!("A polynomial must have at least one coefficient");
        }
        Self {
            coefficients: coeff,
        }
    }

    pub fn evaluate(&self, x: Complex<f64>) -> Complex<f64> {
        let degree = self.degree();
        self.coefficients
            .iter()
            .enumerate()
            .fold(Complex::new(0.0, 0.0), |acc, (i, root)| {
                acc + root * x.powi((degree - i) as i32)
            })
    }

    pub fn derivative(&self) -> Self {
        if self.coefficients.len() == 1 {
            Self::new(vec![Complex::new(0.0, 0.0)])
        } else {
            let mut coeff = Vec::new();
            for i in 1..self.coefficients.len() {
                coeff.push(self.coefficients[i] * Complex::new(i as f64, 0.0));
            }
            Self {
                coefficients: coeff,
            }
        }
    }

    pub fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }

    pub fn add_root(&mut self, root: Complex<f64>) {
        let root_poly = Polynomial::new(vec![Complex::new(1.0, 0.0), -root]);
        *self *= root_poly;
    }

    pub fn add_roots(&mut self, roots: &Vec<Complex<f64>>) {
        for root in roots {
            self.add_root(*root);
        }
    }
}

impl From<(Vec<f64>, Vec<f64>)> for Polynomial {
    fn from((re_root, im_root): (Vec<f64>, Vec<f64>)) -> Self {
        let mut roots = Vec::new();
        for i in 0..re_root.len() {
            roots.push(Complex::new(re_root[i], im_root[i]));
        }
        Self {
            coefficients: roots,
        }
    }
}

impl Mul for Polynomial {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut coeff =
            vec![Complex::new(0.0, 0.0); self.coefficients.len() + rhs.coefficients.len() - 1];
        for i in 0..self.coefficients.len() {
            for j in 0..rhs.coefficients.len() {
                coeff[i + j] += self.coefficients[i] * rhs.coefficients[j];
            }
        }
        Self {
            coefficients: coeff,
        }
    }
}

impl MulAssign for Polynomial {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl PartialEq for Polynomial {
    fn eq(&self, other: &Self) -> bool {
        self.coefficients == other.coefficients
    }
}

impl Debug for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.coefficients
            .iter()
            .enumerate()
            .try_for_each(|(i, coeff)| {
                if i == 0 {
                    write!(f, "{}", coeff)?
                } else {
                    write!(f, " + {} * x^{}", coeff, i)?
                }
                Ok(()) as std::fmt::Result
            })
    }
}
