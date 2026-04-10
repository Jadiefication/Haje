use haje::matrix::Matrix;

#[test]
fn test_matrix_add() {
    let m1 = Matrix::from_fn(|_, _| 1.0);
    let m2 = Matrix::from_fn(|_, _| 2.0);
    let m3: Matrix<f64, 2, 2> = m1 + m2;
    
    for i in 0..2 {
        for j in 0..2 {
            assert_eq!(m3[i][j], 3.0);
        }
    }
}

#[test]
fn test_matrix_rotation() {
    let rad = std::f64::consts::PI / 2.0;
    let rot = Matrix::rotation(rad);
    
    // [[cos(pi/2), -sin(pi/2)], [sin(pi/2), cos(pi/2)]]
    // [[0, -1], [1, 0]]
    assert!((rot[0][0]).abs() < 1e-10);
    assert!((rot[0][1] + 1.0).abs() < 1e-10);
    assert!((rot[1][0] - 1.0).abs() < 1e-10);
    assert!((rot[1][1]).abs() < 1e-10);
}
