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
