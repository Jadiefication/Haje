# Haje

A linear algebra library developed in Rust, focusing on providing a clean and intuitive API for common mathematical operations used in physics simulations and general computations.

## Features

- **Complex Numbers**: Full implementation of complex number arithmetic, including addition, subtraction, multiplication, division, conjugation, magnitude, and exponential functions.
- **Vectors**:
    - `Vec2`, `Vec3`, and `Vec4` types.
    - Support for generic types that implement `RealField`.
    - Standard operations: dot product, magnitude, normalization, and cross product (for `Vec3`).
- **Matrices**:
    - Support for arbitrary dimensions using const generics: `Matrix<T, ROWS, COLS>`.
    - Matrix addition, subtraction, and multiplication.
    - Transposition and determinant calculation for 2x2 and 3x3 matrices.
    - Specialized rotation matrices for 2D and 3D.
- **Physics & Grids**:
    - Wave packet generation (`g_wave_packet`).
    - Discrete Laplacian operator for grid-based calculations.

## Requirements

- Rust (2024 edition)
- Cargo

## Setup

To clone and prepare the project:

```bash
git clone https://github.com/Jadiefication/Haje.git
cd Haje
```

## Usage

### Complex Numbers

```rust
use haje::complex::Complex;

let a = Complex::new(1.0, 2.0);
let b = Complex::new(3.0, 4.0);
let c = a + b;
```

### Vectors

```rust
use haje::vec3::Vec3;

let v1 = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
let v2 = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
let v3 = v1.cross(v2); // Result: Vec3 { x: 0.0, y: 0.0, z: 1.0 }
```

### Matrices

```rust
use haje::matrix::Matrix;

let m1: Matrix<f64, 2, 2> = Matrix::rotation(std::f64::consts::PI / 2.0);
```

## Scripts

- `cargo build`: Compiles the project.
- `cargo test`: Runs the test suite.
- `cargo run`: Runs the main entry point (see `src/main.rs`).

## Project Structure

- `src/lib.rs`: Library entry point.
- `src/complex.rs`: Complex number implementation.
- `src/matrix.rs`: Generic matrix implementation using const generics.
- `src/vec2.rs`, `src/vec3.rs`, `src/vec4.rs`: Vector implementations.
- `src/grid.rs`: Grid-based operations and physics utilities.
- `src/reals.rs`: `RealField` trait for numeric abstraction.
- `tests/`: Comprehensive test suite for all modules.

## License

MIT license
