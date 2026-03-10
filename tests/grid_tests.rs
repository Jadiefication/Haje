use Atom::grid::{grid, g_wave_packet, laplacian};
use Atom::complex::Complex;
use Atom::vec2::Vec2;
use std::f64;

#[test]
fn test_g_wave_packet_zero() {
    let sigma = 1.0;
    let x = 0.0;
    let res = g_wave_packet(x, sigma);
    
    // x = 0.0
    // fraction = exp(-0 / (4 * 1)) / ((2 * PI)^0.25 * sqrt(1))
    // fraction = 1.0 / (2 * PI)^0.25
    let expected_fraction = 1.0 / (2.0 * f64::consts::PI).powf(0.25);
    
    // Complex::new(cos(0), sin(0)) * fraction = (1.0, 0.0) * fraction
    let expected_real = expected_fraction;
    let expected_imaginary = 0.0;
    
    assert!((res.real - expected_real).abs() < 1e-10);
    assert!((res.imaginary - expected_imaginary).abs() < 1e-10);
}

#[test]
fn test_g_wave_packet_non_zero() {
    let sigma = 2.0;
    let x = 1.0;
    let res = g_wave_packet(x, sigma);
    
    let expected_fraction = (-x.powi(2) / (4.0 * sigma.powi(2))).exp() /
        ((2.0 * f64::consts::PI).powf(0.25) * sigma.sqrt());
    
    let expected_real = x.cos() * expected_fraction;
    let expected_imaginary = x.sin() * expected_fraction;
    
    assert!((res.real - expected_real).abs() < 1e-10);
    assert!((res.imaginary - expected_imaginary).abs() < 1e-10);
}

#[test]
fn test_grid_length() {
    let start = 0.0;
    let end = 10.0;
    let amount = 50;
    let res = grid(start, end, amount);
    
    assert_eq!(res.len(), amount as usize);
}

#[test]
fn test_grid_values() {
    let start = -5.0;
    let end = 5.0;
    let amount = 10;
    let res = grid(start, end, amount);
    
    let size = end - start;
    let d_x = size / (amount as f64);
    let sigma = size / 10.0;
    
    for i in 0..amount {
        let x = start + (i as f64) * d_x;
        let expected = g_wave_packet(x, sigma);
        
        assert!((res[i as usize].real - expected.real).abs() < 1e-10);
        assert!((res[i as usize].imaginary - expected.imaginary).abs() < 1e-10);
    }
}

#[test]
fn test_laplacian_internal() {
    // 3x3 grid
    // 1 1 1
    // 1 2 1
    // 1 1 1
    let grid = vec![
        vec![Complex::new(1.0, 0.0), Complex::new(1.0, 0.0), Complex::new(1.0, 0.0)],
        vec![Complex::new(1.0, 0.0), Complex::new(2.0, 0.0), Complex::new(1.0, 0.0)],
        vec![Complex::new(1.0, 0.0), Complex::new(1.0, 0.0), Complex::new(1.0, 0.0)],
    ];
    let center = Vec2 { x: 1, y: 1 };
    let res = laplacian(&center, &grid);

    // left + right + up + down - 4 * center
    // 1 + 1 + 1 + 1 - 4 * 2 = 4 - 8 = -4
    assert!((res.real - (-4.0)).abs() < 1e-10);
    assert!((res.imaginary - 0.0).abs() < 1e-10);
}

#[test]
fn test_laplacian_boundary() {
    // 3x3 grid
    // 1 1 1
    // 1 2 1
    // 1 1 1
    let grid = vec![
        vec![Complex::new(1.0, 0.0), Complex::new(1.0, 0.0), Complex::new(1.0, 0.0)],
        vec![Complex::new(1.0, 0.0), Complex::new(2.0, 0.0), Complex::new(1.0, 0.0)],
        vec![Complex::new(1.0, 0.0), Complex::new(1.0, 0.0), Complex::new(1.0, 0.0)],
    ];
    
    // Test (0, 1) - Left edge
    // Neighbors:
    // left = (0, 1).saturating_sub(1) = (0, 1) -> 1
    // right = (1, 1) -> 2
    // up = (0, 2) -> 1
    // down = (0, 0) -> 1
    // center = (0, 1) -> 1
    // Laplacian: 1 + 2 + 1 + 1 - 4 * 1 = 5 - 4 = 1
    let center = Vec2 { x: 0, y: 1 };
    let res = laplacian(&center, &grid);
    assert!((res.real - 1.0).abs() < 1e-10);

    // Test (1, 0) - Bottom edge (saturating_sub(1) for y=0 is 0, which is center)
    // BUT wait, let's check laplacian implementation in src/grid.rs:
    /*
    let left_val = get(x.saturating_sub(1), y);
    let right_val = get(x + 1, y);
    let up_val = get(x, y + 1);
    let down_val = get(x, y.saturating_sub(1));
    let center_val = get(x, y);
    */
    // If x=1, y=0:
    // left = (0, 0) = 1
    // right = (2, 0) = 1
    // up = (1, 1) = 2
    // down = (1, 0) = 1 (saturating_sub(1) of 0 is 0)
    // center = (1, 0) = 1
    // Laplacian: 1 + 1 + 2 + 1 - 4 * 1 = 5 - 4 = 1
    let center = Vec2 { x: 1, y: 0 };
    let res = laplacian(&center, &grid);
    assert!((res.real - 1.0).abs() < 1e-10);
}

#[test]
fn test_laplacian_corner() {
    // 3x3 grid
    // 1 1 1
    // 1 2 1
    // 1 1 1
    let grid = vec![
        vec![Complex::new(1.0, 0.0), Complex::new(1.0, 0.0), Complex::new(1.0, 0.0)],
        vec![Complex::new(1.0, 0.0), Complex::new(2.0, 0.0), Complex::new(1.0, 0.0)],
        vec![Complex::new(1.0, 0.0), Complex::new(1.0, 0.0), Complex::new(1.0, 0.0)],
    ];

    // Test (0, 0) - Bottom-left corner
    // left = (0, 0) = 1 (saturating_sub(1) of 0 is 0)
    // right = (1, 0) = 1
    // up = (0, 1) = 1
    // down = (0, 0) = 1 (saturating_sub(1) of 0 is 0)
    // center = (0, 0) = 1
    // Laplacian: 1 + 1 + 1 + 1 - 4 * 1 = 0
    let center = Vec2 { x: 0, y: 0 };
    let res = laplacian(&center, &grid);
    assert!((res.real - 0.0).abs() < 1e-10);
    
    // Test (2, 2) - Top-right corner
    // left = (1, 2) = 1
    // right = (3, 2) = 0 (out of bounds)
    // up = (2, 3) = 0 (out of bounds)
    // down = (2, 1) = 1
    // center = (2, 2) = 1
    // Laplacian: 1 + 0 + 0 + 1 - 4 * 1 = 2 - 4 = -2
    let center = Vec2 { x: 2, y: 2 };
    let res = laplacian(&center, &grid);
    assert!((res.real - (-2.0)).abs() < 1e-10);
}
