use std::ops::{Add, Div, Mul, Sub};

use crate::sqrt::Sqrt;

/// A three-dimensional vector.
///
/// Suitable for geometry and physics-style operations in Cartesian space.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct Vec3<T> {
    /// X component.
    pub x: T,
    /// Y component.
    pub y: T,
    /// Z component.
    pub z: T,
}

impl<T> Vec3<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sqrt + Div<Output = T> + Copy + Sub<Output = T>,
{
    /// Returns the dot product of `self` and `other`.
    pub fn dot(&self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Returns the Euclidean length of the vector.
    pub fn magnitude(&self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Returns a normalized vector with unit magnitude.
    pub fn normalize(self) -> Self {
        self / self.magnitude()
    }

    /// Returns the cross product `self × other`.
    ///
    /// The resulting vector is orthogonal to both inputs.
    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl<T> Add for Vec3<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl<T> Sub for Vec3<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl<T> Mul<T> for Vec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl<T> Div<T> for Vec3<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}
