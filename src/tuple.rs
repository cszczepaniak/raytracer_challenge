use std::marker::PhantomData;

use num_traits::Float;

use crate::utils::FuzzyEq;

#[derive(Clone, Copy, Debug)]
pub struct GenericTuple<T, U, const N: usize>
where
    T: Float,
{
    data: [T; N],
    marker: PhantomData<U>,
}

// Default can be generalized for all tuples.
impl<T, U, const N: usize> Default for GenericTuple<T, U, N>
where
    T: Float,
{
    fn default() -> Self {
        Self {
            data: [T::zero(); N],
            marker: Default::default(),
        }
    }
}

// From<T; N> can be generalized for all tuples.
impl<T, U, const N: usize> From<[T; N]> for GenericTuple<T, U, N>
where
    T: Float,
{
    fn from(data: [T; N]) -> Self {
        GenericTuple {
            data,
            marker: PhantomData,
        }
    }
}

// Indexing can be generalized for all tuples.
impl<T, U, const N: usize> std::ops::Index<usize> for GenericTuple<T, U, N>
where
    T: Float,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, U, const N: usize> std::ops::IndexMut<usize> for GenericTuple<T, U, N>
where
    T: Float,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

// FuzzyEq can be generalized for all tuples.
impl<T, U, const N: usize> FuzzyEq for GenericTuple<T, U, N>
where
    T: Float + FuzzyEq,
    U: Copy,
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
impl<T, U, const N: usize> std::ops::Mul<T> for GenericTuple<T, U, N>
where
    T: Float,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut out = Self::Output::default();
        for i in 0..N {
            out[i] = self[i] * rhs;
        }
        out
    }
}

// Scalar division follows from scalar multiplication.
impl<T, U, const N: usize> std::ops::Div<T> for GenericTuple<T, U, N>
where
    T: Float,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let mut out = Self::Output::default();
        for i in 0..N {
            out[i] = self[i] * T::one() / rhs;
        }
        out
    }
}

// Negation follows from scalar multiplication.
impl<T, U, const N: usize> std::ops::Neg for GenericTuple<T, U, N>
where
    T: Float,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut out = Self::Output::default();
        for i in 0..N {
            out[i] = self[i] * -T::one();
        }
        out
    }
}

// Implementation for tuple addition. You get this if your U implements TupleAdd.
pub trait TupleAdd {}
impl<T, U, const N: usize> std::ops::Add for GenericTuple<T, U, N>
where
    T: Float,
    U: TupleAdd,
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
impl<T, U, const N: usize> std::ops::Sub for GenericTuple<T, U, N>
where
    T: Float,
    U: TupleSub,
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

impl<T, U, const N: usize> std::ops::Mul for GenericTuple<T, U, N>
where
    T: Float,
    U: ElementwiseMul,
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

    use super::*;

    struct TestTuple {}

    #[test]
    fn test_mut_indexing() {
        let mut t = GenericTuple::<f64, TestTuple, 4>::default();
        t[0] = 1.0;
        t[1] = 2.0;
        t[2] = 3.0;
        t[3] = 4.0;

        assert_fuzzy_eq!([1.0, 2.0, 3.0, 4.0], t.data);
    }

    #[test]
    fn test_indexing() {
        let t = GenericTuple::<f64, TestTuple, 3>::from([1.0, 2.0, 3.0]);
        assert_fuzzy_eq!([1.0, 2.0, 3.0], t.data);
    }

    #[test]
    fn test_from_array() {
        let t = GenericTuple::<f64, TestTuple, 3>::from([1.0, 2.0, 3.0]);

        assert_eq!(1.0, t[0]);
        assert_eq!(2.0, t[1]);
        assert_eq!(3.0, t[2]);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_indexing_out_of_bounds() {
        let t = GenericTuple::<f64, TestTuple, 4>::default();
        let _ = t[5];
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_mut_indexing_out_of_bounds() {
        let mut t = GenericTuple::<f64, TestTuple, 4>::default();
        t[5] = 6.0;
    }

    #[test]
    fn test_scalar_mult() {
        let mut t = GenericTuple::<f64, TestTuple, 4>::from([1.0, 1.0, 1.0, 1.0]);
        t = t * 0.5;
        assert_fuzzy_eq!([0.5, 0.5, 0.5, 0.5], t.data);
    }

    #[test]
    fn test_div() {
        let mut t = GenericTuple::<f64, TestTuple, 4>::from([1.0, 1.0, 1.0, 1.0]);
        t = t / 0.5;
        assert_fuzzy_eq!([2.0, 2.0, 2.0, 2.0], t.data);
    }

    #[test]
    fn test_neg() {
        let mut t = GenericTuple::<f64, TestTuple, 4>::from([1.0, 1.0, 1.0, 1.0]);
        t = -t;
        assert_fuzzy_eq!([-1.0, -1.0, -1.0, -1.0], t.data);
    }

    #[test]
    fn test_add() {
        impl TupleAdd for TestTuple {}

        let t1 = GenericTuple::<f64, TestTuple, 4>::from([1.0, 1.0, 1.0, 1.0]);
        let t2 = GenericTuple::<f64, TestTuple, 4>::from([1.0, 2.0, 3.0, 4.0]);
        let res = t1 + t2;
        assert_fuzzy_eq!([2.0, 3.0, 4.0, 5.0], res.data);
    }

    #[test]
    fn test_elementwise_mul() {
        impl ElementwiseMul for TestTuple {}

        let t1 = GenericTuple::<f64, TestTuple, 4>::from([-4.0, 3.0, -2.0, 1.0]);
        let t2 = GenericTuple::<f64, TestTuple, 4>::from([1.0, 2.0, 3.0, 4.0]);
        let res = t1 * t2;
        assert_fuzzy_eq!([-4.0, 6.0, -6.0, 4.0], res.data);
    }

    #[test]
    fn test_sub() {
        impl TupleSub for TestTuple {}

        let t1 = GenericTuple::<f64, TestTuple, 4>::from([-4.0, 3.0, -2.0, 1.0]);
        let t2 = GenericTuple::<f64, TestTuple, 4>::from([1.0, 2.0, 3.0, 4.0]);
        let res = t1 - t2;
        assert_fuzzy_eq!([-5.0, 1.0, -5.0, -3.0], res.data);
    }
}
