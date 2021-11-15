use super::vector::Vector;
use crate::{float::Float, tuple::GenericTuple};

pub struct PointStruct {}
type Point<T> = GenericTuple<T, PointStruct, 4>;

impl<T> std::ops::Sub for Point<T>
where
    T: Float,
{
    type Output = Vector<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl<T> Point<T>
where
    T: Float,
{
    fn new(x: T, y: T, z: T) -> Self {
        Point::from([x, y, z, T::identity()])
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
