use std::fmt::Debug;

use crate::utils::FuzzyEq;

use super::vector::Vector;

macro_rules! matrix_type {
    ($name:ident, $size:literal) => {
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        pub struct $name {
            data: [[f64; $size]; $size],
        }

        impl $name {
            pub fn identity() -> Self {
                let mut res: $name = Default::default();
                for i in 0..$size {
                    res[i][i] = 1.0;
                }
                res
            }

            pub fn transpose(&self) -> Self {
                let mut res: $name = Default::default();
                for i in 0..$size {
                    for j in 0..$size {
                        res[i][j] = self[j][i];
                    }
                }
                res
            }
        }

        impl std::ops::Index<usize> for $name {
            type Output = [f64; $size];

            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    0..=$size => &self.data[index],
                    _ => panic!("index out of range"),
                }
            }
        }

        impl std::ops::IndexMut<usize> for $name {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.data[index]
            }
        }

        impl std::ops::Mul for $name {
            type Output = Self;

            fn mul(self, other: Self) -> Self::Output {
                let mut res: $name = Default::default();
                for i in 0..$size {
                    for j in 0..$size {
                        let mut sum = 0.0;
                        for k in 0..$size {
                            sum += self[i][k] * other[k][j];
                        }
                        res[i][j] = sum;
                    }
                }
                res
            }
        }

        impl std::ops::Mul<Option<$name>> for $name {
            type Output = Option<Self>;

            fn mul(self, other: Option<Self>) -> Self::Output {
                if let Some(o) = other {
                    Some(self * o)
                } else {
                    None
                }
            }
        }

        impl std::ops::Mul<$name> for Option<$name> {
            type Output = Self;

            fn mul(self, other: $name) -> Self::Output {
                other * self
            }
        }

        impl std::ops::Mul<Vector> for $name {
            type Output = Vector;

            fn mul(self, rhs: Vector) -> Self::Output {
                let mut res = Vector::new(0.0, 0.0, 0.0);
                for i in 0..$size {
                    let row = self[i];
                    for j in 0..$size {
                        res[i] += row[j] * rhs[j];
                    }
                }
                res
            }
        }

        impl crate::utils::FuzzyEq for $name {
            fn fuzzy_eq(&self, other: Self) -> bool {
                for i in 0..$size {
                    for j in 0..$size {
                        if !self[i][j].fuzzy_eq(other[i][j]) {
                            return false;
                        }
                    }
                }
                true
            }
        }
    };
}

macro_rules! submatrix_ops {
    ($in_type:ident, $in_size:literal, $out_type:ident) => {
        pub fn submatrix(&self, remove_row: usize, remove_col: usize) -> $out_type {
            let mut res: $out_type = Default::default();

            let mut source_row = 0;
            let mut source_col = 0;
            let mut target_row = 0;
            let mut target_col = 0;

            while source_row < $in_size {
                if source_row == remove_row {
                    source_row += 1;
                    continue;
                }
                while source_col < $in_size {
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
            for i in 0..$in_size {
                res += self[0][i] * self.cofactor(0, i);
            }
            res
        }
    };
}

matrix_type!(Matrix2, 2);
matrix_type!(Matrix3, 3);
matrix_type!(Matrix4, 4);

impl Matrix2 {
    pub fn determinant(&self) -> f64 {
        self[0][0] * self[1][1] - self[1][0] * self[0][1]
    }
}

impl Matrix3 {
    submatrix_ops!(Matrix3, 3, Matrix2);
}

impl Matrix4 {
    submatrix_ops!(Matrix4, 4, Matrix3);

    pub fn is_invertible(&self) -> bool {
        self.determinant().fuzzy_ne(0.0)
    }

    pub fn inverse(&self) -> Self {
        if !self.is_invertible() {
            panic!("matrix is not invertible")
        }
        let mut res: Matrix4 = Default::default();
        let det = self.determinant();
        for i in 0..4 {
            for j in 0..4 {
                // transpose as we go
                res[j][i] = self.cofactor(i, j) / det;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_fuzzy_eq, utils::FuzzyEq};

    use super::*;

    macro_rules! matrix {
        ($r1:expr, $r2:expr) => {
            Matrix2 { data: [$r1, $r2] }
        };
        ($r1:expr, $r2:expr,) => {
            Matrix2 { data: [$r1, $r2] }
        };
        ($r1:expr, $r2:expr, $r3:expr) => {
            Matrix3 {
                data: [$r1, $r2, $r3],
            }
        };
        ($r1:expr, $r2:expr, $r3:expr,) => {
            Matrix3 {
                data: [$r1, $r2, $r3],
            }
        };
        ($r1:expr, $r2:expr, $r3:expr, $r4:expr) => {
            Matrix4 {
                data: [$r1, $r2, $r3, $r4],
            }
        };
        ($r1:expr, $r2:expr, $r3:expr, $r4:expr,) => {
            Matrix4 {
                data: [$r1, $r2, $r3, $r4],
            }
        };
    }

    #[test]
    fn matrix_equality() {
        let m1 = matrix!([1.0, 2.0], [3.0, 4.0]);
        let m2 = matrix!([1.0, 2.0], [3.0, 4.0]);
        assert_fuzzy_eq!(m1, m2);

        let m1 = matrix!([1.0, 2.0, 3.0], [3.0, 4.0, 5.0], [5.0, 6.0, 7.0]);
        let m2 = matrix!([1.0, 2.0, 3.0], [3.0, 4.0, 5.0], [5.0, 6.0, 7.0]);
        assert_fuzzy_eq!(m1, m2);

        let m1 = matrix!(
            [1.0, 2.0, 3.0, 4.0],
            [3.0, 4.0, 5.0, 6.0],
            [6.0, 7.0, 8.0, 0.3],
            [9.0, 10.0, 11.0, 12.0],
        );
        let m2 = matrix!(
            [1.0, 2.0, 3.0, 4.0],
            [3.0, 4.0, 5.0, 6.0],
            [6.0, 7.0, 8.0, 3.0 / 10.0],
            [9.0, 10.0, 11.0, 12.0],
        );
        assert_fuzzy_eq!(m1, m2);
    }

    #[test]
    fn matrix_multiplication_with_vector() {
        let m = matrix!([1.0, 2.0], [3.0, 4.0]);
        let v = Vector::new(1.0, 2.0, 0.0);
        let exp = Vector::new(5.0, 11.0, 0.0);
        let res = m * v;

        assert_fuzzy_eq!(exp, res);

        let m = matrix!([1.0, 2.0, -5.0], [3.0, 4.0, 1.0], [0.5, 0.6, 1.0]);
        let v = Vector::new(1.0, 2.0, 3.0);
        let exp = Vector::new(-10.0, 14.0, 4.7);
        let res = m * v;

        assert_fuzzy_eq!(exp, res);

        let m = matrix!(
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        );
        let v = Vector::new(1.0, 2.0, 3.0);
        let exp = Vector::new(14.0, 22.0, 32.0);
        let res = m * v;

        assert_fuzzy_eq!(exp, res);
    }

    #[test]
    fn matrix_multiplication_with_matrix() {
        let m1 = matrix!([1.0, 2.0], [3.0, 4.0]);
        let m2 = matrix!([-1.0, -2.0], [3.0, 4.0]);
        let exp = matrix!([5.0, 6.0], [9.0, 10.0]);

        assert_fuzzy_eq!(exp, m1 * m2);

        let m1 = matrix!([1.0, 2.0, -5.0], [3.0, 4.0, 1.0], [0.5, 0.6, 1.0]);
        let m2 = matrix!([-1.0, -2.0, 1.0], [3.0, 4.0, 2.0], [1.0, 1.0, 2.5]);
        let exp = matrix!([0.0, 1.0, -7.5], [10.0, 11.0, 13.5], [2.3, 2.4, 4.2]);

        assert_fuzzy_eq!(exp, m1 * m2);

        let m1 = matrix!(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        );
        let m2 = matrix!(
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        );
        let exp = matrix!(
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        );

        assert_fuzzy_eq!(exp, m1 * m2);
    }

    #[test]
    fn matrix_identity_multiplication() {
        let m = matrix!([1.0, 2.0], [3.0, 4.0]);
        assert_fuzzy_eq!(Matrix2::identity() * m, m);
        assert_fuzzy_eq!(m * Matrix2::identity(), m);

        let m = matrix!([1.0, 2.0, -5.0], [3.0, 4.0, 1.0], [0.5, 0.6, 1.0]);
        assert_fuzzy_eq!(Matrix3::identity() * m, m);
        assert_fuzzy_eq!(m * Matrix3::identity(), m);

        let m = matrix!(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        );
        assert_fuzzy_eq!(Matrix4::identity() * m, m);
        assert_fuzzy_eq!(m * Matrix4::identity(), m);
    }

    #[test]
    fn matrix_transpose() {
        let m1 = matrix!([1.0, 2.0], [3.0, 4.0]);
        let m2 = matrix!([1.0, 3.0], [2.0, 4.0]);
        assert_fuzzy_eq!(m1.transpose(), m2);

        let m1 = matrix!([1.0, 2.0, -5.0], [3.0, 4.0, 1.0], [0.5, 0.6, 1.0]);
        let m2 = matrix!([1.0, 3.0, 0.5], [2.0, 4.0, 0.6], [-5.0, 1.0, 1.0]);
        assert_fuzzy_eq!(m1.transpose(), m2);

        let m1 = matrix!(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        );
        let m2 = matrix!(
            [1.0, 5.0, 9.0, 5.0],
            [2.0, 6.0, 8.0, 4.0],
            [3.0, 7.0, 7.0, 3.0],
            [4.0, 8.0, 6.0, 2.0],
        );
        assert_fuzzy_eq!(m1.transpose(), m2);
    }

    #[test]
    fn matrix_determinant() {
        let m = matrix!([1.0, 5.0], [-3.0, 2.0]);
        assert_fuzzy_eq!(17.0, m.determinant());

        let m = matrix!([1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]);
        let c00 = m.cofactor(0, 0);
        let c01 = m.cofactor(0, 1);
        let c02 = m.cofactor(0, 2);

        let det = m.determinant();

        assert_fuzzy_eq!(56.0, c00);
        assert_fuzzy_eq!(12.0, c01);
        assert_fuzzy_eq!(-46.0, c02);
        assert_fuzzy_eq!(-196.0, det);

        let m = matrix!(
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        );

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
        let m = matrix!([1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, 3.0]);
        let exp = matrix!([-3.0, 2.0], [0.0, 6.0]);
        assert_fuzzy_eq!(exp, m.submatrix(0, 2));
        let exp = matrix!([1.0, 0.0], [0.0, 3.0]);
        assert_fuzzy_eq!(exp, m.submatrix(1, 1));

        let m = matrix!(
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 3.0, 4.0, 5.0],
            [3.0, 4.0, 5.0, 6.0],
            [4.0, 5.0, 6.0, 7.0],
        );
        let exp = matrix!([1.0, 3.0, 4.0], [3.0, 5.0, 6.0], [4.0, 6.0, 7.0],);
        assert_fuzzy_eq!(exp, m.submatrix(1, 1));
        let exp = matrix!([1.0, 2.0, 4.0], [2.0, 3.0, 5.0], [3.0, 4.0, 6.0],);
        assert_fuzzy_eq!(exp, m.submatrix(3, 2));
    }

    #[test]
    fn matrix_minor() {
        let m = matrix!([3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]);
        let sub = m.submatrix(1, 0);
        let det = sub.determinant();
        let minor = m.minor(1, 0);

        assert_fuzzy_eq!(det, minor);

        let m = matrix!(
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 3.0, 4.0, 5.0],
            [3.0, 4.0, 5.0, 6.0],
            [4.0, 5.0, 6.0, 7.0],
        );
        let sub = m.submatrix(2, 3);
        let det = sub.determinant();
        let minor = m.minor(2, 3);

        assert_fuzzy_eq!(det, minor);
    }

    #[test]
    fn matrix_cofactor() {
        let m = matrix!([3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]);
        let minor1 = m.minor(0, 0);
        let minor2 = m.minor(1, 0);

        let cofactor1 = m.cofactor(0, 0);
        let cofactor2 = m.cofactor(1, 0);

        assert_fuzzy_eq!(-12.0, minor1);
        assert_fuzzy_eq!(-12.0, cofactor1);
        assert_fuzzy_eq!(25.0, minor2);
        assert_fuzzy_eq!(-25.0, cofactor2);

        let m = matrix!(
            [-1.0, 2.0, 3.0, 4.0],
            [6.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, -7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        );
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
        let m = matrix!(
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        );

        let det = m.determinant();
        assert_fuzzy_eq!(-2120.0, det);
        assert!(m.is_invertible());

        let m = matrix!(
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        );

        let det = m.determinant();
        assert_fuzzy_eq!(0.0, det);
        assert!(!m.is_invertible());
    }

    #[test]
    fn matrix_invert() {
        let m = matrix!(
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        );

        let det = m.determinant();
        let c23 = m.cofactor(2, 3);
        let c32 = m.cofactor(3, 2);

        let exp = matrix!(
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        );
        let act = m.inverse();
        assert_fuzzy_eq!(532.0, det);
        assert_fuzzy_eq!(-160.0, c23);
        assert_fuzzy_eq!(-160.0 / 532.0, act[3][2]);
        assert_fuzzy_eq!(105.0, c32);
        assert_fuzzy_eq!(105.0 / 532.0, act[2][3]);
        assert_fuzzy_eq!(exp, act);
        assert_fuzzy_eq!(Matrix4::identity(), act * m);
        assert_fuzzy_eq!(Matrix4::identity(), m * act);
    }

    #[test]
    #[should_panic]
    fn matrix_inverse_uninvertible() {
        let m = matrix!(
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        );
        let _ = m.inverse();
    }

    #[test]
    fn matrix_inverse_undoes_a_product() {
        let m1 = matrix!(
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        );
        let m2 = matrix!(
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        );

        let m3 = m1 * m2;

        let act = m3 * m2.inverse();
        assert_fuzzy_eq!(m1, act);
    }
}
