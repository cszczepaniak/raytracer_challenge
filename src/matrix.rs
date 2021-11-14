use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub},
};

use crate::utils::FuzzyEq;

use super::vector::Vector;

// Define a marker trait for the types we'd like to constrain our matrix to.
// This trait inherits from some subset of the ops that are possible for both
// f32 and f64 so we can use +, -, *, /, etc. in our generic implementations.
// We also add an identity function, which we need for the identity matrices.
pub trait Float:
    Default
    + Copy
    + Add<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Sub<Output = Self>
    + Neg<Output = Self>
    + AddAssign<Self>
{
    fn identity() -> Self;
}
impl Float for f32 {
    fn identity() -> Self {
        1.0
    }
}
impl Float for f64 {
    fn identity() -> Self {
        1.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Matrix<T, const N: usize>
where
    T: Float,
{
    data: [[T; N]; N],
}

// We can generalize the following trait implementations for _all_ matrices,
// regardless of type and size.

impl<T, const N: usize> From<[[T; N]; N]> for Matrix<T, N>
where
    T: Float,
{
    fn from(data: [[T; N]; N]) -> Self {
        Matrix { data }
    }
}

impl<T, const N: usize> Default for Matrix<T, N>
where
    T: Float,
{
    fn default() -> Self {
        let def: [[T; N]; N] = [[T::default(); N]; N];
        Self::from(def)
    }
}

impl<T, const N: usize> Index<usize> for Matrix<T, N>
where
    T: Float,
{
    type Output = [T; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Matrix<T, N>
where
    T: Float,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T, const N: usize> Mul for Matrix<T, N>
where
    T: Float,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res: Self::Output = Default::default();
        for i in 0..N {
            for j in 0..N {
                let mut sum = T::default();
                for k in 0..N {
                    sum += self[i][k] * rhs[k][j];
                }
                res[i][j] = sum;
            }
        }
        res
    }
}

impl<T, const N: usize> Mul<T> for Matrix<T, N>
where
    T: Float,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut res: Self::Output = Default::default();
        for i in 0..N {
            for j in 0..N {
                res[i][j] = rhs * res[i][j];
            }
        }
        res
    }
}

impl<T, const N: usize> FuzzyEq for Matrix<T, N>
where
    T: Float + FuzzyEq,
{
    fn fuzzy_eq(&self, other: Self) -> bool {
        for i in 0..N {
            for j in 0..N {
                if self[i][j].fuzzy_ne(other[i][j]) {
                    return false;
                }
            }
        }
        true
    }
}

impl<T, const N: usize> Matrix<T, N>
where
    T: Float,
{
    pub fn identity() -> Self {
        let mut res = Self::default();
        for i in 0..N {
            res[i][i] = T::identity();
        }
        res
    }

    pub fn transpose(&self) -> Self {
        let mut res = Self::default();
        for i in 0..N {
            for j in 0..N {
                res[i][j] = self[j][i];
            }
        }
        res
    }
}

// The implementation for determinant is special for 2x2.
// Bigger matricies have a more general solution.
impl<T> Matrix<T, 2>
where
    T: Float,
{
    pub fn determinant(&self) -> T {
        self[0][0] * self[1][1] - self[1][0] * self[0][1]
    }
}

// Unfortunately, const generics don't allow us to do things like N-1, so the next best way to
// not have to write this code twice is to define it as a macro.
macro_rules! submatrix_ops {
    ($size:literal, $down_size:literal) => {
        impl<T> Matrix<T, $size>
        where
            T: Float + FuzzyEq,
        {
            pub fn submatrix(&self, remove_row: usize, remove_col: usize) -> Matrix<T, $down_size> {
                let mut res: Matrix<T, $down_size> = Default::default();

                let mut source_row = 0;
                let mut source_col = 0;
                let mut target_row = 0;
                let mut target_col = 0;

                while source_row < $size {
                    if source_row == remove_row {
                        source_row += 1;
                        continue;
                    }
                    while source_col < $size {
                        if source_col == remove_col {
                            source_col += 1;
                            continue;
                        }

                        res[target_row][target_col] = self[source_row][source_col];
                        source_col += 1;
                        target_col += 1;
                    }
                    source_row += 1;
                    target_row += 1;
                    source_col = 0;
                    target_col = 0;
                }

                res
            }

            pub fn minor(&self, remove_row: usize, remove_col: usize) -> T {
                self.submatrix(remove_row, remove_col).determinant()
            }

            pub fn cofactor(&self, remove_row: usize, remove_col: usize) -> T {
                let minor = self.minor(remove_row, remove_col);
                if (remove_row + remove_col) % 2 == 0 {
                    minor
                } else {
                    -minor
                }
            }

            pub fn determinant(&self) -> T {
                let mut res = T::default();
                for i in 0..$size {
                    res += self[0][i] * self.cofactor(0, i);
                }
                res
            }

            pub fn is_invertible(&self) -> bool {
                self.determinant().fuzzy_ne(T::default())
            }

            pub fn inverse(&self) -> Self {
                if !self.is_invertible() {
                    panic!("matrix is not invertible")
                }
                let mut res: Self = Default::default();
                let det = self.determinant();
                for i in 0..$size {
                    for j in 0..$size {
                        // transpose as we go
                        res[j][i] = self.cofactor(i, j) / det;
                    }
                }
                res
            }
        }
    };
}

submatrix_ops!(4, 3);
submatrix_ops!(3, 2);

// We only have 4-element vectors, so let's only implement matrix-vector
// multiplication for 4x4 matrices.
impl Mul<Vector> for Matrix<f64, 4> {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        let mut res = Vector::new(0.0, 0.0, 0.0);
        for i in 0..4 {
            let row = self[i];
            for j in 0..4 {
                res[i] += row[j] * (rhs[j]);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_fuzzy_eq, utils::FuzzyEq};

    use super::*;

    #[test]
    fn matrix_equality() {
        let m1 = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
        let m2 = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
        assert_fuzzy_eq!(m1, m2);

        let m1 = Matrix::from([[1.0, 2.0, 3.0], [3.0, 4.0, 5.0], [5.0, 6.0, 7.0]]);
        let m2 = Matrix::from([[1.0, 2.0, 3.0], [3.0, 4.0, 5.0], [5.0, 6.0, 7.0]]);
        assert_fuzzy_eq!(m1, m2);

        let m1 = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [3.0, 4.0, 5.0, 6.0],
            [6.0, 7.0, 8.0, 0.3],
            [9.0, 10.0, 11.0, 12.0],
        ]);
        let m2 = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [3.0, 4.0, 5.0, 6.0],
            [6.0, 7.0, 8.0, 3.0 / 10.0],
            [9.0, 10.0, 11.0, 12.0],
        ]);
        assert_fuzzy_eq!(m1, m2);
    }

    #[test]
    fn matrix_multiplication_with_vector() {
        let m = Matrix::<f64, 4>::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let v = Vector::new(1.0, 2.0, 3.0);
        let exp = Vector::new(14.0, 22.0, 32.0);
        let res = m * v;

        assert_fuzzy_eq!(exp, res);
    }

    #[test]
    fn matrix_multiplication_with_matrix() {
        let m1 = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
        let m2 = Matrix::from([[-1.0, -2.0], [3.0, 4.0]]);
        let exp = Matrix::from([[5.0, 6.0], [9.0, 10.0]]);

        assert_fuzzy_eq!(exp, m1 * m2);

        let m1 = Matrix::from([[1.0, 2.0, -5.0], [3.0, 4.0, 1.0], [0.5, 0.6, 1.0]]);
        let m2 = Matrix::from([[-1.0, -2.0, 1.0], [3.0, 4.0, 2.0], [1.0, 1.0, 2.5]]);
        let exp = Matrix::from([[0.0, 1.0, -7.5], [10.0, 11.0, 13.5], [2.3, 2.4, 4.2]]);

        assert_fuzzy_eq!(exp, m1 * m2);

        let m1 = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2 = Matrix::from([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        let exp = Matrix::from([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);

        assert_fuzzy_eq!(exp, m1 * m2);
    }

    #[test]
    fn matrix_identity_multiplication() {
        let m = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
        assert_fuzzy_eq!(Matrix::identity() * m, m);
        assert_fuzzy_eq!(m * Matrix::identity(), m);

        let m = Matrix::from([[1.0, 2.0, -5.0], [3.0, 4.0, 1.0], [0.5, 0.6, 1.0]]);
        assert_fuzzy_eq!(Matrix::identity() * m, m);
        assert_fuzzy_eq!(m * Matrix::identity(), m);

        let m = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        assert_fuzzy_eq!(Matrix::identity() * m, m);
        assert_fuzzy_eq!(m * Matrix::identity(), m);
    }

    #[test]
    fn matrix_transpose() {
        let m1 = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
        let m2 = Matrix::from([[1.0, 3.0], [2.0, 4.0]]);
        assert_fuzzy_eq!(m1.transpose(), m2);

        let m1 = Matrix::from([[1.0, 2.0, -5.0], [3.0, 4.0, 1.0], [0.5, 0.6, 1.0]]);
        let m2 = Matrix::from([[1.0, 3.0, 0.5], [2.0, 4.0, 0.6], [-5.0, 1.0, 1.0]]);
        assert_fuzzy_eq!(m1.transpose(), m2);

        let m1 = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2 = Matrix::from([
            [1.0, 5.0, 9.0, 5.0],
            [2.0, 6.0, 8.0, 4.0],
            [3.0, 7.0, 7.0, 3.0],
            [4.0, 8.0, 6.0, 2.0],
        ]);
        assert_fuzzy_eq!(m1.transpose(), m2);
    }

    #[test]
    fn matrix_determinant() {
        let m = Matrix::from([[1.0, 5.0], [-3.0, 2.0]]);
        assert_fuzzy_eq!(17.0, m.determinant());

        let m = Matrix::from([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);
        let c00 = m.cofactor(0, 0);
        let c01 = m.cofactor(0, 1);
        let c02 = m.cofactor(0, 2);

        let det = m.determinant();

        assert_fuzzy_eq!(56.0, c00);
        assert_fuzzy_eq!(12.0, c01);
        assert_fuzzy_eq!(-46.0, c02);
        assert_fuzzy_eq!(-196.0, det);

        let m = Matrix::from([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);

        let c00 = m.cofactor(0, 0);
        let c01 = m.cofactor(0, 1);
        let c02 = m.cofactor(0, 2);
        let c03 = m.cofactor(0, 3);

        let det = m.determinant();

        assert_fuzzy_eq!(690.0, c00);
        assert_fuzzy_eq!(447.0, c01);
        assert_fuzzy_eq!(210.0, c02);
        assert_fuzzy_eq!(51.0, c03);
        assert_fuzzy_eq!(-4071.0, det);
    }

    #[test]
    fn matrix_submatrix() {
        let m = Matrix::from([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, 3.0]]);
        let exp = Matrix::from([[-3.0, 2.0], [0.0, 6.0]]);
        assert_fuzzy_eq!(exp, m.submatrix(0, 2));
        let exp = Matrix::from([[1.0, 0.0], [0.0, 3.0]]);
        assert_fuzzy_eq!(exp, m.submatrix(1, 1));

        let m = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 3.0, 4.0, 5.0],
            [3.0, 4.0, 5.0, 6.0],
            [4.0, 5.0, 6.0, 7.0],
        ]);
        let exp = Matrix::from([[1.0, 3.0, 4.0], [3.0, 5.0, 6.0], [4.0, 6.0, 7.0]]);
        assert_fuzzy_eq!(exp, m.submatrix(1, 1));
        let exp = Matrix::from([[1.0, 2.0, 4.0], [2.0, 3.0, 5.0], [3.0, 4.0, 6.0]]);
        assert_fuzzy_eq!(exp, m.submatrix(3, 2));
    }

    #[test]
    fn matrix_minor() {
        let m = Matrix::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        let sub = m.submatrix(1, 0);
        let det = sub.determinant();
        let minor = m.minor(1, 0);

        assert_fuzzy_eq!(det, minor);

        let m = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 3.0, 4.0, 5.0],
            [3.0, 4.0, 5.0, 6.0],
            [4.0, 5.0, 6.0, 7.0],
        ]);
        let sub = m.submatrix(2, 3);
        let det = sub.determinant();
        let minor = m.minor(2, 3);

        assert_fuzzy_eq!(det, minor);
    }

    #[test]
    fn matrix_cofactor() {
        let m = Matrix::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        let minor1 = m.minor(0, 0);
        let minor2 = m.minor(1, 0);

        let cofactor1 = m.cofactor(0, 0);
        let cofactor2 = m.cofactor(1, 0);

        assert_fuzzy_eq!(-12.0, minor1);
        assert_fuzzy_eq!(-12.0, cofactor1);
        assert_fuzzy_eq!(25.0, minor2);
        assert_fuzzy_eq!(-25.0, cofactor2);

        let m = Matrix::from([
            [-1.0, 2.0, 3.0, 4.0],
            [6.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, -7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let minor1 = m.minor(1, 3);
        let minor2 = m.minor(1, 2);

        let cofactor1 = m.cofactor(1, 3);
        let cofactor2 = m.cofactor(1, 2);

        assert_fuzzy_eq!(-188.0, minor1);
        assert_fuzzy_eq!(-188.0, cofactor1);
        assert_fuzzy_eq!(16.0, minor2);
        assert_fuzzy_eq!(-16.0, cofactor2);
    }

    #[test]
    fn matrix_invertibility() {
        let m = Matrix::from([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);

        let det = m.determinant();
        assert_fuzzy_eq!(-2120.0, det);
        assert!(m.is_invertible());

        let m = Matrix::from([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        let det = m.determinant();
        assert_fuzzy_eq!(0.0, det);
        assert!(!m.is_invertible());
    }

    #[test]
    fn matrix_invert() {
        let m = Matrix::from([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);

        let det = m.determinant();
        let c23 = m.cofactor(2, 3);
        let c32 = m.cofactor(3, 2);

        let exp = Matrix::from([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);
        let act = m.inverse();
        assert_fuzzy_eq!(532.0, det);
        assert_fuzzy_eq!(-160.0, c23);
        assert_fuzzy_eq!(-160.0 / 532.0, act[3][2]);
        assert_fuzzy_eq!(105.0, c32);
        assert_fuzzy_eq!(105.0 / 532.0, act[2][3]);
        assert_fuzzy_eq!(exp, act);
        assert_fuzzy_eq!(Matrix::<f64, 4>::identity(), act * m);
        assert_fuzzy_eq!(Matrix::<f64, 4>::identity(), m * act);
    }

    #[test]
    #[should_panic]
    fn matrix_inverse_uninvertible() {
        let m = Matrix::from([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);
        let _ = m.inverse();
    }

    #[test]
    fn matrix_inverse_undoes_a_product() {
        let m1 = Matrix::from([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);
        let m2 = Matrix::from([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);

        let m3 = m1 * m2;

        let act = m3 * m2.inverse();
        assert_fuzzy_eq!(m1, act);
    }
}
