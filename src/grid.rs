use crate::complex::Complex;
use crate::vec2::Vec2;
use std::f64;
use std::ops::{Div, Index};
use crate::reals::RealField;
use crate::vec3::Vec3;

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

fn pot_energy_fn<T: RealField>(pos: Vec3<T>) -> T where f32: Div<T, Output = T> {
    2.0f32.exp() / pos.magnitude()
}

pub fn update(grid: &mut Vec<Vec<Complex>>, t: f64, d_t: f64) {
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };
    for x in 0..rows {
        let vec = &grid[x];
        for y in 0..cols {
            let complex = grid[x][y];
            let energy = pot_energy_fn(Vec3 { x: x as f32, y: y as f32, z: 0.0 });

            let left = if x > 0 { grid[x - 1][y] } else { Complex::new(0.0, 0.0) };
            let right = if x + 1 < rows { grid[x + 1][y] } else { Complex::new(0.0, 0.0) };
            let up = if y + 1 < cols { grid[x][y + 1] } else { Complex::new(0.0, 0.0) };
            let down = if y > 0 { grid[x][y - 1] } else { Complex::new(0.0, 0.0) };

            let laplacian = left + right + up + down - complex * 4.0;
            let mut hamilton = laplacian * -(PLANCK / (2.0 * MASS));
            hamilton.real = hamilton.real + energy as f64;

            grid[x][y] = Complex { real: 1.0, imaginary: 0.0 }
                - Complex { real: 0.0, imaginary: 1.0 } * ((hamilton * d_t) / PLANCK);
        }
    }
}