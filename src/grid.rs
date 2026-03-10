use crate::complex::Complex;
use crate::vec2::Vec2;
use std::{f64, panic};

const MASS: f64 = 9.11e-31;
const PLANCK: f64 = 1.054e-34;
const K_0: i8 = 1;
const PI: f64 = f64::consts::PI;
const E: f64 = f64::consts::E;

pub fn grid(start: f64, end: f64, amount: i32) -> Vec<Complex> {
    let size = end - start;
    let d_x = size / (amount as f64);
    let sigma = size / 10.0;
    let mut vec: Vec<Complex> = vec![];
    for i in 0..amount {
        let x = start + (i as f64) * d_x;
        let pos = g_wave_packet(x, sigma);
        vec.push(pos);
    }
    vec
}

pub fn g_wave_packet(x: f64, sigma: f64) -> Complex {
    let fraction = (-x.powi(2)/(4.0*sigma.powi(2))).exp() /
        ((2.0*PI).powf(1.0/4.0) * sigma.sqrt());
    Complex::new(x.cos(), x.sin()) * fraction
}

pub fn laplacian(center: &Vec2<usize>, grid: &Vec<Vec<Complex>>) -> Complex {
    let x = center.x;
    let y = center.y;
    let max_x = grid.len();
    let max_y = if max_x > 0 { grid[0].len() } else { 0 };

    // Helper closure to safely get a value or 0 if out of bounds
    let get = |i: usize, j: usize| -> Complex {
        if i < max_x && j < max_y {
            grid[i][j]
        } else {
            Complex { real: 0.0, imaginary: 0.0 }
        }
    };

    let left_val = get(x.saturating_sub(1), y);
    let right_val = get(x + 1, y);
    let up_val = get(x, y + 1);
    let down_val = get(x, y.saturating_sub(1));
    let center_val = get(x, y);

    left_val + right_val + up_val + down_val - center_val * 4.0
}