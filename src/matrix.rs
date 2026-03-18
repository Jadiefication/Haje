use crate::complex::Complex;
use crate::vec2::Vec2;
use std::ops::{Add, Index, IndexMut, Mul, Sub};

pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
    data: [[T; COLS]; ROWS],
}

impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS>
{
    pub fn from_fn<F>(mut f: F) -> Self
    where
        F: FnMut(usize, usize) -> T,
    {
        Matrix {
            data: std::array::from_fn(|i| {
                std::array::from_fn(|j| {
                    f(i, j)
                })
            }),
        }
    }

    pub fn transpose(&self) -> Matrix<T, COLS, ROWS>
    where
        T: Clone
    {
        Matrix::from_fn(|j, i| self[i][j].clone())
    }
}

impl<T, const ROWS: usize, const COLS: usize> Index<usize> for Matrix<T, ROWS, COLS> {
    type Output = [T; COLS];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const ROWS: usize, const COLS: usize> IndexMut<usize> for Matrix<T, ROWS, COLS> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T, const ROWS: usize, const COLS: usize> Add for Matrix<T, ROWS, COLS>
where
    for<'a> &'a T: Add<Output = T>,
{
    type Output = Matrix<T, ROWS, COLS>;

    fn add(self, rhs: Self) -> Self::Output {
        let left = self.data;
        let right = rhs.data;
        Matrix::from_fn(|i, j| {
            &left[i][j] + &right[i][j]
        })
    }
}

impl<T, const ROWS: usize, const COLS: usize> Sub for Matrix<T, ROWS, COLS>
where
    for<'a> &'a T: Sub<Output = T>,
{
    type Output = Matrix<T, ROWS, COLS>;

    fn sub(self, rhs: Self) -> Self::Output {
        let left = self.data;
        let right = rhs.data;
        Matrix::from_fn(|i, j| {
            &left[i][j] - &right[i][j]
        })
    }
}

impl<T, const ROWS: usize, const COLS: usize> Mul<f64> for Matrix<T, ROWS, COLS>
where
    for<'a> &'a T: Mul<&'a f64, Output = f64>,
{
    type Output = Matrix<f64, ROWS, COLS>;

    fn mul(self, rhs: f64) -> Self::Output {
        let left = self.data;
        Matrix::from_fn(|i, j| {
            &left[i][j] * &rhs
        })
    }
}

impl<T, const ROWS: usize, const COLS: usize> Mul<Complex> for Matrix<T, ROWS, COLS>
where
    for<'a> &'a T: Mul<&'a Complex, Output = Complex>,
{
    type Output = Matrix<Complex, ROWS, COLS>;

    fn mul(self, rhs: Complex) -> Self::Output {
        let left = self.data;
        Matrix::from_fn(|i, j| {
            &left[i][j] * &rhs
        })
    }
}

impl<T, N, const ROWS: usize, const COLS: usize, const K: usize>
Mul<Matrix<N, COLS, K>> for Matrix<T, ROWS, COLS>
where
    for<'a> &'a T: Mul<&'a N, Output = N> ,
    N: Add<Output = N> + Default,
{
    type Output = Matrix<N, ROWS, K>;

    fn mul(self, rhs: Matrix<N, COLS, K>) -> Self::Output {
        let mut result = Matrix::from_fn(|_, _| N::default());

        for i in 0..ROWS {
            for j in 0..K {
                let mut sum = N::default();
                for k in 0..COLS {
                    sum = sum + &self.data[i][k] * &rhs.data[k][j];
                }
                result.data[i][j] = sum;
            }
        }

        result
    }
}

impl Matrix<f64, 2, 2> {
    pub fn rotation(radians: f64) -> Self {
        Matrix {
            data: [
                [radians.cos(), -radians.sin()],
                [radians.sin(), radians.cos()]
            ]
        }
    }
}

impl Matrix<f64, 3, 3> {
    pub fn r_x(radians: f64) -> Self {
        Matrix {
            data: [
                [1.0, 0.0, 0.0],
                [0.0, radians.cos(), -radians.sin()],
                [0.0, radians.sin(), radians.cos()]
            ]
        }
    }

    pub fn r_y(radians: f64) -> Self {
        Matrix {
            data: [
                [radians.cos(), 0.0, radians.sin()],
                [0.0, 1.0, 0.0],
                [-radians.sin(), 0.0, radians.cos()]
            ]
        }
    }

    pub fn r_z(radians: f64) -> Self {
        Matrix {
            data: [
                [radians.cos(), -radians.sin(), 0.0],
                [radians.sin(), radians.cos(), 0.0],
                [0.0, 0.0, 1.0]
            ]
        }
    }
}

impl<T> Matrix<T, 3, 3> {
    pub fn det3(&self) -> T
    where
        for<'a> &'a T: Mul<Output = &'a T> + Sub<Output = &'a T> + From<i32> + Add<Output = T>,
    {
        // 3x3 determinant: cofactor expansion
        let a = &self[0][0];
        let b = &self[0][1];
        let c = &self[0][2];
        let d = &self[1][0];
        let e = &self[1][1];
        let f = &self[1][2];
        let g = &self[2][0];
        let h = &self[2][1];
        let i = &self[2][2];
        a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)
    }
}

impl<T, N> Mul<Vec2<T>> for Matrix<N, 2, 2>
where
    T: Mul<N, Output = T> + Add<Output = T>, for<'a> &'a T: Mul<&'a N, Output = T>
{
    type Output = Vec2<T>;

    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        let ref x = rhs.x;
        let ref y = rhs.y;
        Vec2 {
            x: x * &self[0][0] + y * &self[0][1],
            y: x * &self[1][0] + y * &self[1][1],
        }
    }
}

impl<T> Matrix<T, 2, 2> {
    pub fn det2(&self) -> T
    where
            for<'a> &'a T: Mul<Output = &'a T> + Sub<Output = T> + From<i32> + Add<Output = T>,
    {
        let a = &self[0][0];
        let b = &self[0][1];
        let c = &self[1][0];
        let d = &self[1][1];
        a * d - b * c
    }
}