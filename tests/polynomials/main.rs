use newton_fractal::Polynomial;
use num_complex::Complex;

#[test]
fn polynomial_root() {
    let mut poly = Polynomial::default();
    poly.add_roots(&vec![Complex::new(1.0, 0.0), Complex::new(2.0, 0.0)]);
    println!("{:?}", poly);
    assert_eq!(
        poly.evaluate(Complex::new(1.0, 0.0)),
        Complex::new(0.0, 0.0)
    );
    assert_eq!(
        poly.evaluate(Complex::new(2.0, 0.0)),
        Complex::new(0.0, 0.0)
    );
}

#[test]
fn default_polynomial_derivative() {
    let poly = Polynomial::default();
    let poly_derivative = poly.derivative();
    assert_eq!(
        poly_derivative,
        Polynomial::new(vec![Complex::new(0.0, 0.0)])
    );
}

#[test]
fn polynomial_derivative() {
    let poly = Polynomial::new(vec![
        Complex::new(1.0, 0.0),
        Complex::new(2.0, 0.0),
        Complex::new(3.0, 0.0),
    ]);
    let poly_derivative = poly.derivative();
    assert_eq!(
        poly_derivative,
        Polynomial::new(vec![Complex::new(2.0, 0.0), Complex::new(6.0, 0.0)])
    );
}

#[test]
fn polynomial_degree() {
    let poly = Polynomial::new(vec![
        Complex::new(1.0, 0.0),
        Complex::new(2.0, 0.0),
        Complex::new(3.0, 0.0),
    ]);
    assert_eq!(poly.degree(), 2);

    let poly = Polynomial::new(vec![Complex::new(1.0, 0.0)]);
    assert_eq!(poly.degree(), 0);
}
