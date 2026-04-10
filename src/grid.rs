use crate::complex::Complex;
use crate::vec2::Vec2;
use std::f64;

pub const PLANCK: f64 = 1.054e-34;
pub const PI: f64 = f64::consts::PI;
pub const E: f64 = f64::consts::E;

pub fn g_wave_packet(x: f64, sigma: f64) -> Complex {
    let fraction = (-x.powi(2)/(4.0*sigma.powi(2))).exp() /
        ((2.0*PI).powf(1.0/4.0) * sigma.sqrt());
    Complex::new(x.cos(), x.sin()) * fraction
}

pub fn laplacian(center: &Vec2<usize>, grid: &Vec<Vec<Complex>>) -> Complex {
    let x = center.x;
    let y = center.y;

    let left_val = get(grid, x.saturating_sub(1), y);
    let right_val = get(grid, x + 1, y);
    let up_val = get(grid, x, y + 1);
    let down_val = get(grid, x, y.saturating_sub(1));
    let center_val = get(grid, x, y);

    left_val + right_val + up_val + down_val - center_val * 4.0
}

fn get(grid: &Vec<Vec<Complex>>, i: usize, j: usize) -> Complex {
    let max_x = grid.len();
    let max_y = if max_x > 0 { grid[0].len() } else { 0 };
    if i < max_x && j < max_y {
        grid[i][j]
    } else {
        Complex { real: 0.0, imaginary: 0.0 }
    }
}