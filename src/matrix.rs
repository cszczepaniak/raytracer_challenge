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
            fn fuzzy_eq(&self, other: &Self) -> bool {
                use crate::utils::f64_fuzzy_eq;
                for i in 0..$size {
                    for j in 0..$size {
                        if !f64_fuzzy_eq(self[i][j], other[i][j]) {
                            return false;
                        }
                    }
                }
                true
            }
        }
    };
}

matrix_type!(Matrix2, 2);
matrix_type!(Matrix3, 3);
matrix_type!(Matrix4, 4);

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
        assert_fuzzy_eq!(m1, &m2);

        let m1 = matrix!([1.0, 2.0, 3.0], [3.0, 4.0, 5.0], [5.0, 6.0, 7.0]);
        let m2 = matrix!([1.0, 2.0, 3.0], [3.0, 4.0, 5.0], [5.0, 6.0, 7.0]);
        assert_fuzzy_eq!(m1, &m2);

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
        assert_fuzzy_eq!(m1, &m2);
    }

    #[test]
    fn matrix_multiplication_with_vector() {
        let m = matrix!([1.0, 2.0], [3.0, 4.0]);
        let v = Vector::new(1.0, 2.0, 0.0);
        let exp = Vector::new(5.0, 11.0, 0.0);
        let res = m * v;

        assert_fuzzy_eq!(exp, &res);

        let m = matrix!([1.0, 2.0, -5.0], [3.0, 4.0, 1.0], [0.5, 0.6, 1.0]);
        let v = Vector::new(1.0, 2.0, 3.0);
        let exp = Vector::new(-10.0, 14.0, 4.7);
        let res = m * v;

        assert_fuzzy_eq!(exp, &res);

        let m = matrix!(
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        );
        let v = Vector::new(1.0, 2.0, 3.0);
        let exp = Vector::new(14.0, 22.0, 32.0);
        let res = m * v;

        assert_fuzzy_eq!(exp, &res);
    }

    #[test]
    fn matrix_multiplication_with_matrix() {
        let m1 = matrix!([1.0, 2.0], [3.0, 4.0]);
        let m2 = matrix!([-1.0, -2.0], [3.0, 4.0]);
        let exp = matrix!([5.0, 6.0], [9.0, 10.0]);

        assert_fuzzy_eq!(m1 * m2, &exp);

        let m1 = matrix!([1.0, 2.0, -5.0], [3.0, 4.0, 1.0], [0.5, 0.6, 1.0]);
        let m2 = matrix!([-1.0, -2.0, 1.0], [3.0, 4.0, 2.0], [1.0, 1.0, 2.5]);
        let exp = matrix!([0.0, 1.0, -7.5], [10.0, 11.0, 13.5], [2.3, 2.4, 4.2]);

        assert_fuzzy_eq!(m1 * m2, &exp);

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

        assert_fuzzy_eq!(m1 * m2, &exp);
    }

    #[test]
    fn matrix_identity_multiplication() {
        let m1 = matrix!([1.0, 2.0], [3.0, 4.0]);
        assert_fuzzy_eq!(Matrix2::identity() * m1, &m1);
        assert_fuzzy_eq!(m1 * Matrix2::identity(), &m1);

        let m1 = matrix!([1.0, 2.0, -5.0], [3.0, 4.0, 1.0], [0.5, 0.6, 1.0]);
        assert_fuzzy_eq!(Matrix3::identity() * m1, &m1);
        assert_fuzzy_eq!(m1 * Matrix3::identity(), &m1);

        let m1 = matrix!(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        );
        assert_fuzzy_eq!(Matrix4::identity() * m1, &m1);
        assert_fuzzy_eq!(m1 * Matrix4::identity(), &m1);
    }
}
