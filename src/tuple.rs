use std::{
    marker::PhantomData,
    ops::{self, Add, Div, Index, IndexMut, Mul, Neg},
};

use crate::utils::FuzzyEq;

#[derive(Clone, Copy, Debug)]
pub struct Tuple<T, const N: usize> {
    data: [f64; N],
    marker: PhantomData<T>,
}

// Default can be generalized for all tuples.
impl<T, const N: usize> Default for Tuple<T, N> {
    fn default() -> Self {
        Self {
            data: [0.0; N],
            marker: Default::default(),
        }
    }
}

// From<[T; N]> can be generalized for all tuples.
impl<T, const N: usize> From<[f64; N]> for Tuple<T, N> {
    fn from(data: [f64; N]) -> Self {
        Tuple {
            data,
            marker: PhantomData,
        }
    }
}

// Indexing can be generalized for all tuples.
impl<T, const N: usize> Index<usize> for Tuple<T, N> {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Tuple<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

// FuzzyEq can be generalized for all tuples.
impl<T, const N: usize> FuzzyEq for Tuple<T, N>
where
    T: Copy,
{
    fn fuzzy_eq(&self, other: Self) -> bool {
        for i in 0..N {
            if self[i].fuzzy_ne(other[i]) {
                return false;
            }
        }
        true
    }
}

// Scalar multiplication can be generalized for all tuples.
impl<T, const N: usize> Mul<f64> for Tuple<T, N> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut out = Self::Output::default();
        for i in 0..N {
            out[i] = self[i] * rhs;
        }
        out
    }
}

// Scalar division follows from scalar multiplication.
impl<T, const N: usize> Div<f64> for Tuple<T, N> {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let mut out = Self::Output::default();
        for i in 0..N {
            out[i] = self[i] * 1.0 / rhs;
        }
        out
    }
}

// Negation follows from scalar multiplication.
impl<T, const N: usize> Neg for Tuple<T, N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut out = Self::Output::default();
        for i in 0..N {
            out[i] = self[i] * -1.0;
        }
        out
    }
}

// Implementation for tuple addition. You get this if your U implements TupleAdd.
pub trait TupleAdd {}
impl<T, const N: usize> Add for Tuple<T, N>
where
    T: TupleAdd,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = Self::Output::default();
        for i in 0..N {
            out[i] = self[i] + rhs[i];
        }
        out
    }
}

// Implementation for tuple subtraction. You get this if your U implements TupleSub.
// TODO if you want an output type other than Self, I _think_ we'd need GATs which are not stable yet...
// For now, for Point subtraction, we'll have to implement it explicitly.
pub trait TupleSub {}
impl<T, const N: usize> ops::Sub for Tuple<T, N>
where
    T: TupleSub,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut out = Self::Output::default();
        for i in 0..N {
            out[i] = self[i] - rhs[i];
        }
        out
    }
}

// Implementation for elementwise multiplication. You get this if your U implements ElementwiseMul.
pub trait ElementwiseMul {}

impl<T, const N: usize> ops::Mul for Tuple<T, N>
where
    T: ElementwiseMul,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut out = Self::Output::default();
        for i in 0..N {
            out[i] = self[i] * rhs[i];
        }
        out
    }
}

#[cfg(test)]
mod tests {

    use crate::assert_fuzzy_eq;
    use crate::utils::FuzzyEq;

    use super::*;

    #[derive(Clone, Copy, Debug)]
    struct TestTuple {}
    type Test = Tuple<TestTuple, 4>;

    #[test]
    fn test_mut_indexing() {
        let mut t = Test::default();
        t[0] = 1.0;
        t[1] = 2.0;
        t[2] = 3.0;
        t[3] = 4.0;

        assert_fuzzy_eq!([1.0, 2.0, 3.0, 4.0], t.data);
    }

    #[test]
    fn test_indexing() {
        let t = Test::from([1.0, 2.0, 3.0, 4.0]);
        assert_fuzzy_eq!(1.0, t[0]);
        assert_fuzzy_eq!(2.0, t[1]);
        assert_fuzzy_eq!(3.0, t[2]);
        assert_fuzzy_eq!(4.0, t[3]);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_indexing_out_of_bounds() {
        let t = Test::default();
        let _ = t[5];
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_mut_indexing_out_of_bounds() {
        let mut t = Test::default();
        t[5] = 6.0;
    }

    #[test]
    fn test_scalar_mult() {
        let mut t = Test::from([1.0, 1.0, 1.0, 1.0]);
        t = t * 0.5;
        assert_fuzzy_eq!(Test::from([0.5, 0.5, 0.5, 0.5]), t);
    }

    #[test]
    fn test_div() {
        let mut t = Test::from([1.0, 1.0, 1.0, 1.0]);
        t = t / 0.5;
        assert_fuzzy_eq!(Test::from([2.0, 2.0, 2.0, 2.0]), t);
    }

    #[test]
    fn test_neg() {
        let mut t = Test::from([1.0, 1.0, 1.0, 1.0]);
        t = -t;
        assert_fuzzy_eq!(Test::from([-1.0, -1.0, -1.0, -1.0]), t);
    }

    #[test]
    fn test_add() {
        impl TupleAdd for TestTuple {}

        let t1 = Test::from([1.0, 1.0, 1.0, 1.0]);
        let t2 = Test::from([1.0, 2.0, 3.0, 4.0]);
        let res = t1 + t2;
        assert_fuzzy_eq!(Test::from([2.0, 3.0, 4.0, 5.0]), res);
    }

    #[test]
    fn test_elementwise_mul() {
        impl ElementwiseMul for TestTuple {}

        let t1 = Test::from([-4.0, 3.0, -2.0, 1.0]);
        let t2 = Test::from([1.0, 2.0, 3.0, 4.0]);
        let res = t1 * t2;
        assert_fuzzy_eq!(Test::from([-4.0, 6.0, -6.0, 4.0]), res);
    }

    #[test]
    fn test_sub() {
        impl TupleSub for TestTuple {}

        let t1 = Test::from([-4.0, 3.0, -2.0, 1.0]);
        let t2 = Test::from([1.0, 2.0, 3.0, 4.0]);
        let res = t1 - t2;
        assert_fuzzy_eq!(Test::from([-5.0, 1.0, -5.0, -3.0]), res);
    }
}
