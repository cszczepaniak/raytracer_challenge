use std::{
    fmt::Debug,
    ops::{Index, IndexMut, Mul},
};

use crate::{tuple::Tuple, utils::FuzzyEq};

#[derive(Debug, Copy, Clone)]
pub struct Matrix<const N: usize> {
    data: [[f64; N]; N],
}

// We can generalize the following trait implementations for _all_ matrices,
// regardless of type and size.

impl<const N: usize> From<[[f64; N]; N]> for Matrix<N> {
    fn from(data: [[f64; N]; N]) -> Self {
        Matrix { data }
    }
}

impl<const N: usize> Default for Matrix<N> {
    fn default() -> Self {
        Self::from([[0.0; N]; N])
    }
}

impl<const N: usize> Index<usize> for Matrix<N> {
    type Output = [f64; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const N: usize> IndexMut<usize> for Matrix<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const N: usize> Mul for Matrix<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res: Self::Output = Default::default();
        for i in 0..N {
            for j in 0..N {
                let mut sum = 0.0;
                for k in 0..N {
                    sum = sum + self[i][k] * rhs[k][j];
                }
                res[i][j] = sum;
            }
        }
        res
    }
}

impl<const N: usize> Mul<f64> for Matrix<N> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut res: Self::Output = Default::default();
        for i in 0..N {
            for j in 0..N {
                res[i][j] = rhs * res[i][j];
            }
        }
        res
    }
}

impl<const N: usize> FuzzyEq for Matrix<N> {
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

impl<const N: usize> Matrix<N> {
    pub fn identity() -> Self {
        let mut res = Self::default();
        for i in 0..N {
            res[i][i] = 1.0;
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
impl Matrix<2> {
    pub fn determinant(&self) -> f64 {
        self[0][0] * self[1][1] - self[1][0] * self[0][1]
    }
}

// Unfortunately, const generics don't allow us to do things like N-1, so the next best way to
// not have to write this code twice is to define it as a macro.
macro_rules! submatrix_ops {
    ($size:literal, $down_size:literal) => {
        impl Matrix<$size> {
            pub fn submatrix(&self, remove_row: usize, remove_col: usize) -> Matrix<$down_size> {
                let mut res: Matrix<$down_size> = Default::default();

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

            pub fn minor(&self, remove_row: usize, remove_col: usize) -> f64 {
                self.submatrix(remove_row, remove_col).determinant()
            }

            pub fn cofactor(&self, remove_row: usize, remove_col: usize) -> f64 {
                let minor = self.minor(remove_row, remove_col);
                if (remove_row + remove_col) % 2 == 0 {
                    minor
                } else {
                    -minor
                }
            }

            pub fn determinant(&self) -> f64 {
                let mut res = 0.0;
                for i in 0..$size {
                    res = res + self[0][i] * self.cofactor(0, i);
                }
                res
            }

            pub fn is_invertible(&self) -> bool {
                self.determinant().fuzzy_ne(0.0)
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

pub enum Rotation {
    X,
    Y,
    Z,
}

pub enum Shear {
    XY(f64),
    XZ(f64),
    YX(f64),
    YZ(f64),
    ZX(f64),
    ZY(f64),
}

impl Matrix<4> {
    #[rustfmt::skip]
    pub fn translate(x: f64, y: f64, z: f64) -> Self {
        Matrix::from([
            [1.0, 0.0, 0.0, x  ],
            [0.0, 1.0, 0.0, y  ],
            [0.0, 0.0, 1.0, z  ],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    #[rustfmt::skip]
    pub fn scale(x: f64, y: f64, z: f64) -> Self {
        Matrix::from([
            [x,   0.0, 0.0, 0.0],
            [0.0, y,   0.0, 0.0],
            [0.0, 0.0, z,   0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotate(dir: Rotation, theta: f64) -> Self {
        match dir {
            Rotation::X => Matrix::rotate_x(theta),
            Rotation::Y => Matrix::rotate_y(theta),
            Rotation::Z => Matrix::rotate_z(theta),
        }
    }

    #[rustfmt::skip]
    pub fn rotate_x(theta: f64) -> Self {
        Matrix::from([
            [1.0, 0.0,         0.0,          0.0],
            [0.0, theta.cos(), -theta.sin(), 0.0],
            [0.0, theta.sin(), theta.cos(),  0.0],
            [0.0, 0.0,         0.0,          1.0],
        ])
    }

    #[rustfmt::skip]
    pub fn rotate_y(theta: f64) -> Self {
        Matrix::from([
            [theta.cos(),  0.0, theta.sin(), 0.0],
            [0.0,          1.0, 0.0,         0.0],
            [-theta.sin(), 0.0, theta.cos(), 0.0],
            [0.0,          0.0, 0.0,         1.0],
        ])
    }

    #[rustfmt::skip]
    pub fn rotate_z(theta: f64) -> Self {
        Matrix::from([
            [theta.cos(), -theta.sin(), 0.0, 0.0],
            [theta.sin(), theta.cos(),  0.0, 0.0],
            [0.0,         0.0,          1.0, 0.0],
            [0.0,         0.0,          0.0, 1.0],
        ])
    }

    pub fn shear(shears: &[Shear]) -> Self {
        let mut res = Self::identity();
        for sh in shears {
            match sh {
                &Shear::XY(v) => res[0][1] = v,
                &Shear::XZ(v) => res[0][2] = v,
                &Shear::YX(v) => res[1][0] = v,
                &Shear::YZ(v) => res[1][2] = v,
                &Shear::ZX(v) => res[2][0] = v,
                &Shear::ZY(v) => res[2][1] = v,
            };
        }
        res
    }
}

// We only have 4-element vectors and points so let's only implement matrix-tuple
// multiplication between 4x4 matrices and 4 element tuples.
impl<T> Mul<Tuple<T, 4>> for Matrix<4> {
    type Output = Tuple<T, 4>;

    fn mul(self, rhs: Tuple<T, 4>) -> Self::Output {
        let mut res = Self::Output::default();
        for i in 0..4 {
            let row = self[i];
            for j in 0..4 {
                res[i] = res[i] + row[j] * rhs[j];
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_4};

    use crate::{assert_fuzzy_eq, point::Point, utils::FuzzyEq, vector::Vector};

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
        let m = Matrix::from([
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
    fn matrix_multiplication_with_point() {
        let m = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let v = Point::new(1.0, 2.0, 3.0);
        let exp = Point::new(18.0, 24.0, 33.0);
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
        assert_fuzzy_eq!(Matrix::<4>::identity(), act * m);
        assert_fuzzy_eq!(Matrix::<4>::identity(), m * act);
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

    #[test]
    fn matrix_translate() {
        let p = Point::new(1.0, 2.0, 3.0);
        let t = Matrix::translate(1.0, -2.0, 3.0);
        let res = t * p;
        assert_fuzzy_eq!(Point::new(2.0, 0.0, 6.0), res);

        // Multiplying by the inverse should bring us back
        let res = t.inverse() * res;
        assert_fuzzy_eq!(p, res);
    }

    #[test]
    fn matrix_scale_point() {
        let p = Point::new(1.0, 2.0, 3.0);
        let t = Matrix::scale(1.0, -2.0, 3.0);
        let res = t * p;
        assert_fuzzy_eq!(Point::new(1.0, -4.0, 9.0), res);

        // Multiplying by the inverse should bring us back
        let res = t.inverse() * res;
        assert_fuzzy_eq!(p, res);
    }

    #[test]
    fn matrix_scale_vector() {
        let p = Vector::new(1.0, 2.0, 3.0);
        let t = Matrix::scale(1.0, -2.0, 3.0);
        let res = t * p;
        assert_fuzzy_eq!(Vector::new(1.0, -4.0, 9.0), res);

        // Multiplying by the inverse should bring us back
        let res = t.inverse() * res;
        assert_fuzzy_eq!(p, res);
    }

    macro_rules! matrix_rotate_x {
        ($tuple:ident, $name:ident) => {
            #[test]
            fn $name() {
                let p = $tuple::new(0.0, 1.0, 0.0);
                let t = Matrix::rotate(Rotation::X, FRAC_PI_2);
                let res = t * p;
                assert_fuzzy_eq!($tuple::new(0.0, 0.0, 1.0), res);

                // Multiplying by the inverse should bring us back
                let res = t.inverse() * res;
                assert_fuzzy_eq!(p, res);

                let p = $tuple::new(0.0, 1.0, 0.0);
                let t = Matrix::rotate(Rotation::X, FRAC_PI_4);
                let res = t * p;
                assert_fuzzy_eq!($tuple::new(0.0, FRAC_1_SQRT_2, FRAC_1_SQRT_2), res);

                // Multiplying by the inverse should bring us back
                let res = t.inverse() * res;
                assert_fuzzy_eq!(p, res);
            }
        };
    }

    matrix_rotate_x!(Vector, matrix_rotate_x_vect);
    matrix_rotate_x!(Point, matrix_rotate_x_point);

    macro_rules! matrix_rotate_y {
        ($tuple:ident, $name:ident) => {
            #[test]
            fn $name() {
                let p = $tuple::new(0.0, 0.0, 1.0);
                let t = Matrix::rotate(Rotation::Y, FRAC_PI_2);
                let res = t * p;
                assert_fuzzy_eq!($tuple::new(1.0, 0.0, 0.0), res);

                // Multiplying by the inverse should bring us back
                let res = t.inverse() * res;
                assert_fuzzy_eq!(p, res);

                let p = $tuple::new(0.0, 0.0, 1.0);
                let t = Matrix::rotate(Rotation::Y, FRAC_PI_4);
                let res = t * p;
                assert_fuzzy_eq!($tuple::new(FRAC_1_SQRT_2, 0.0, FRAC_1_SQRT_2), res);

                // Multiplying by the inverse should bring us back
                let res = t.inverse() * res;
                assert_fuzzy_eq!(p, res);
            }
        };
    }

    matrix_rotate_y!(Vector, matrix_rotate_y_vect);
    matrix_rotate_y!(Point, matrix_rotate_y_point);

    macro_rules! matrix_rotate_z {
        ($tuple:ident, $name:ident) => {
            #[test]
            fn $name() {
                let p = $tuple::new(0.0, 1.0, 0.0);
                let t = Matrix::rotate(Rotation::Z, FRAC_PI_2);
                let res = t * p;
                assert_fuzzy_eq!($tuple::new(-1.0, 0.0, 0.0), res);

                // Multiplying by the inverse should bring us back
                let res = t.inverse() * res;
                assert_fuzzy_eq!(p, res);

                let p = $tuple::new(0.0, 1.0, 0.0);
                let t = Matrix::rotate(Rotation::Z, FRAC_PI_4);
                let res = t * p;
                assert_fuzzy_eq!($tuple::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0), res);

                // Multiplying by the inverse should bring us back
                let res = t.inverse() * res;
                assert_fuzzy_eq!(p, res);
            }
        };
    }

    matrix_rotate_z!(Vector, matrix_rotate_z_vect);
    matrix_rotate_z!(Point, matrix_rotate_z_point);

    #[test]
    fn matrix_shearing_x() {
        let transform = Matrix::shear(&[Shear::XY(1.0)]);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_fuzzy_eq!(transform * p, Point::new(5.0, 3.0, 4.0));

        let transform = Matrix::shear(&[Shear::XZ(1.0)]);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_fuzzy_eq!(transform * p, Point::new(6.0, 3.0, 4.0));
    }

    #[test]
    fn matrix_shearing_y() {
        let transform = Matrix::shear(&[Shear::YX(1.0)]);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_fuzzy_eq!(transform * p, Point::new(2.0, 5.0, 4.0));

        let transform = Matrix::shear(&[Shear::YZ(1.0)]);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_fuzzy_eq!(transform * p, Point::new(2.0, 7.0, 4.0));
    }

    #[test]
    fn matrix_shearing_z() {
        let transform = Matrix::shear(&[Shear::ZX(1.0)]);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_fuzzy_eq!(transform * p, Point::new(2.0, 3.0, 6.0));

        let transform = Matrix::shear(&[Shear::ZY(1.0)]);
        let p = Point::new(2.0, 3.0, 4.0);

        assert_fuzzy_eq!(transform * p, Point::new(2.0, 3.0, 7.0));
    }

    #[test]
    fn matrix_transforms_in_sequence() {
        let p = Point::new(1.0, 0.0, 1.0);
        let a = Matrix::rotate(Rotation::X, FRAC_PI_2);
        let b = Matrix::scale(5.0, 5.0, 5.0);
        let c = Matrix::translate(10.0, 5.0, 7.0);

        let p2 = a * p;
        assert_fuzzy_eq!(Point::new(1.0, -1.0, 0.0), p2);

        let p3 = b * p2;
        assert_fuzzy_eq!(Point::new(5.0, -5.0, 0.0), p3);

        let p4 = c * p3;
        assert_fuzzy_eq!(Point::new(15.0, 0.0, 7.0), p4);

        let transform = c * b * a;
        let p5 = transform * p;
        assert_fuzzy_eq!(Point::new(15.0, 0.0, 7.0), p5);
    }
}
