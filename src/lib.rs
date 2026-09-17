//! `haje` is a compact math library centered around linear algebra primitives,
//! complex arithmetic, and small quantum-oriented helpers.
//!
//! The crate is designed for educational and simulation-oriented workloads where
//! straightforward APIs and explicit data layout are preferred over heavy abstraction.
//!
//! Module guide:
//! - [`complex`]: a `f64`-based [`complex::Complex`] type with arithmetic operators
//!   and common polar/exponential helpers.
//! - [`reals`]: the [`reals::RealField`] trait used by generic vector types.
//! - [`matrix`]: const-generic [`matrix::Matrix`] for fixed-size matrices,
//!   including addition/subtraction/multiplication, transpose, and selected
//!   determinant/rotation helpers.
//! - [`calculus`]: utility functions used by the tests and examples (Gaussian
//!   wave packet, discrete Laplacian, and `relu`).
//! - [`mod@vec`]: 2D/3D/4D vector types for geometric computations.
//! - [`quantum`]: predefined single- and two-qubit gates and qubit state containers.
//!
//! # Example
//! ```rust
//! use haje::complex::Complex;
//! use haje::matrix::Matrix;
//!
//! let rot = Matrix::<f64, 2, 2>::rotation(std::f64::consts::PI / 2.0);
//! let z = Complex::new(1.0, 2.0).conj();
//!
//! assert!((rot[0][1] + 1.0).abs() < 1e-10);
//! assert_eq!(z, Complex::new(1.0, -2.0));
//! ```

#[macro_use]
pub(crate) mod macros;

/// Calculus and grid-based helper functions.
pub mod calculus;
/// Complex number utilities and operators.
pub mod complex;
/// Const-generic matrix type and operations.
pub mod matrix;
#[cfg(feature = "gpu")]
mod priv_gpu;
/// Quantum gates and qubit state utilities.
pub mod quantum;
mod sqrt;
/// Vector types (`Vec2`, `Vec3`, `Vec4`).
pub mod vec;
