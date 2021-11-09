use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Neg};

// Tuple is the base type of vectors, points, colors, etc.
// There are a few traits that are common between all tuples:
//   1. Adding two tuples is always elementwise and always produces the same kind of tuple
//   2. Scalar multiplication (and therefore division) is the same for all tuples
//   3. Negation of a tuple is always elementwise
// These will be implemented here, and tuples of a particular type will use the generic type T to
// implement their specific attributes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tuple<T>(pub f64, pub f64, pub f64, pub f64, pub PhantomData<T>);

impl<T> Add for Tuple<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
            PhantomData,
        )
    }
}

impl<T> Mul<f64> for Tuple<T> {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self(
            self.0 * other,
            self.1 * other,
            self.2 * other,
            self.3 * other,
            PhantomData,
        )
    }
}

impl<T> Mul<Tuple<T>> for f64 {
    type Output = Tuple<T>;

    fn mul(self, other: Tuple<T>) -> Self::Output {
        Tuple(
            self * other.0,
            self * other.1,
            self * other.2,
            self * other.3,
            PhantomData,
        )
    }
}

impl<T> Div<f64> for Tuple<T> {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        (1.0 / other) * self
    }
}

impl<T> Neg for Tuple<T> {
    type Output = Self;

    fn neg(self) -> Self {
        -1.0 * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_tuples() {
        let v1 = Tuple(1.0, 2.0, 3.0, 4.0, PhantomData::<f64>);
        let v2 = Tuple(2.0, 3.0, -4.0, 5.0, PhantomData);
        assert_eq!(v1 + v2, Tuple(3.0, 5.0, -1.0, 9.0, PhantomData));
    }

    #[test]
    fn test_negate_tuple() {
        assert_eq!(
            -Tuple(1.0, -2.0, 3.0, -4.0, PhantomData::<f64>),
            Tuple(-1.0, 2.0, -3.0, 4.0, PhantomData::<f64>),
        )
    }

    #[test]
    fn test_tuple_scalar_mult() {
        assert_eq!(
            Tuple(1.0, 2.0, 3.0, 4.0, PhantomData::<()>) * 2.0,
            Tuple(2.0, 4.0, 6.0, 8.0, PhantomData::<()>),
        );
        assert_eq!(
            2.0 * Tuple(1.0, 2.0, 3.0, 4.0, PhantomData::<()>),
            Tuple(2.0, 4.0, 6.0, 8.0, PhantomData::<()>)
        )
    }

    #[test]
    fn test_vec_scalar_div() {
        assert_eq!(
            Tuple(1.0, 2.0, 3.0, 4.0, PhantomData::<()>) / 2.0,
            Tuple(0.5, 1.0, 1.5, 2.0, PhantomData::<()>),
        )
    }
}
