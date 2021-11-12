use std::ops::{Add, Div, Mul, Neg};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tuple<const N: usize> {
    data: [f64; N],
}

impl<const N: usize> Add for Tuple<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut new = Tuple::<N> { data: [0.0; N] };
        for i in 0..N {
            new.data[i] = self.data[i] + other.data[i];
        }
        new
    }
}

impl<const N: usize> Mul<Tuple<N>> for f64 {
    type Output = Tuple<N>;

    fn mul(self, other: Tuple<N>) -> Self::Output {
        let mut new = Tuple::<N> { data: [0.0; N] };
        for i in 0..N {
            new.data[i] = other.data[i] * self;
        }
        new
    }
}

impl<const N: usize> Mul<f64> for Tuple<N> {
    type Output = Tuple<N>;

    fn mul(self, other: f64) -> Self::Output {
        other * self
    }
}

impl<const N: usize> Div<f64> for Tuple<N> {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        (1.0 / other) * self
    }
}

impl<const N: usize> Neg for Tuple<N> {
    type Output = Self;

    fn neg(self) -> Self {
        -1.0 * self
    }
}

// Tuple is the base type of vectors, points, colors, etc.
// There are a few traits that are common between all tuples:
//   1. Adding two tuples is always elementwise and always produces the same kind of tuple
//   2. Scalar multiplication (and therefore division) is the same for all tuples
//   3. Negation of a tuple is always elementwise
// These will be implemented here, and tuples of a particular type will use the generic type T to
// implement their specific attributes.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_tuples() {
        let v1 = Tuple {
            data: [1.0, 2.0, 3.0, 4.0],
        };
        let v2 = Tuple {
            data: [2.0, 3.0, -4.0, 5.0],
        };
        assert_eq!(
            v1 + v2,
            Tuple {
                data: [3.0, 5.0, -1.0, 9.0]
            }
        );
    }

    #[test]
    fn test_negate_tuple() {
        assert_eq!(
            -Tuple {
                data: [1.0, -2.0, 3.0, -4.0]
            },
            Tuple {
                data: [-1.0, 2.0, -3.0, 4.0]
            },
        )
    }

    #[test]
    fn test_tuple_scalar_mult() {
        assert_eq!(
            Tuple {
                data: [1.0, 2.0, 3.0, 4.0]
            } * 2.0,
            Tuple {
                data: [2.0, 4.0, 6.0, 8.0]
            },
        );
        assert_eq!(
            2.0 * Tuple {
                data: [1.0, 2.0, 3.0, 4.0]
            },
            Tuple {
                data: [2.0, 4.0, 6.0, 8.0]
            }
        )
    }

    #[test]
    fn test_vec_scalar_div() {
        assert_eq!(
            Tuple {
                data: [1.0, 2.0, 3.0, 4.0]
            } / 2.0,
            Tuple {
                data: [0.5, 1.0, 1.5, 2.0]
            },
        )
    }
}
