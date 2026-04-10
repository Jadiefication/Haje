use haje::complex::Complex;

#[test]
fn test_complex_add() {
    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);
    let c = a + b;
    assert_eq!(c.real, 4.0);
    assert_eq!(c.imaginary, 6.0);
}

#[test]
fn test_complex_mul() {
    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);
    let c = a * b;
    // (1+2i)(3+4i) = 3 + 4i + 6i - 8 = -5 + 10i
    assert_eq!(c.real, -5.0);
    assert_eq!(c.imaginary, 10.0);
}

#[test]
fn test_complex_mag() {
    let a = Complex::new(3.0, 4.0);
    assert_eq!(a.mag(), 5.0);
}
