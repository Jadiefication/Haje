use std::ops::{Add, Div, Mul, Sub};

use crate::sqrt::Sqrt;

/// A four-dimensional vector.
///
/// The fields are named `r/g/b/a` to align with common color-vector usage,
/// but the type can also represent generic 4D coordinates.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct Vec4<T> {
    /// First component (often interpreted as red channel).
    pub r: T,
    /// Second component (often interpreted as green channel).
    pub g: T,
    /// Third component (often interpreted as blue channel).
    pub b: T,
    /// Fourth component (often interpreted as alpha channel).
    pub a: T,
}

impl<T> Vec4<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sqrt + Copy + Div<Output = T>,
{
    /// Returns the dot product of `self` and `other`.
    pub fn dot(self, other: Self) -> T {
        self.r * other.r + self.g * other.g + self.b * other.b + self.a * other.a
    }

    /// Returns the Euclidean length of the vector.
    pub fn magnitude(self) -> T {
        (self.r * self.r + self.g * self.g + self.b * self.b + self.a * self.a).sqrt()
    }

    /// Returns a normalized vector with unit magnitude.
    ///
    /// This operation does not guard against zero-length vectors.
    pub fn normalize(self) -> Self {
        self / self.magnitude()
    }
}

impl<T> Add for Vec4<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self { r: self.r + rhs.r, g: self.g + rhs.g, b: self.b + rhs.b, a: self.a + rhs.a }
    }
}

impl<T> Sub for Vec4<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self { r: self.r - rhs.r, g: self.g - rhs.g, b: self.b - rhs.b, a: self.a - rhs.a }
    }
}

impl<T> Mul<T> for Vec4<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self { r: self.r * rhs, g: self.g * rhs, b: self.b * rhs, a: self.a * rhs }
    }
}

impl<T> Div<T> for Vec4<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self { r: self.r / rhs, g: self.g / rhs, b: self.b / rhs, a: self.a / rhs }
    }
}
