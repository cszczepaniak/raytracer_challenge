use num_traits::Float;

use super::vector::Vector;
use crate::tuple::Tuple;

#[derive(Clone, Copy, Debug)]
pub struct PointStruct {}
pub type Point<T> = Tuple<T, PointStruct, 4>;

impl<T> std::ops::Sub for Point<T>
where
    T: Float,
{
    type Output = Vector<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl<T> Into<Vector<T>> for Point<T>
where
    T: Float,
{
    fn into(self) -> Vector<T> {
        Vector::new(self[0], self[1], self[2])
    }
}

impl<T> Point<T>
where
    T: Float,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Point::from([x, y, z, T::one()])
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_fuzzy_eq;
    use crate::utils::FuzzyEq;

    use super::*;

    #[test]
    fn test_point_sub() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let p2 = Point::new(2.0, -2.0, 2.0);
        let res = p1 - p2;
        assert_fuzzy_eq!(Vector::new(-1.0, 4.0, 1.0), res);
    }
}
