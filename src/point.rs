use super::vector::Vector;
use crate::tuple::Tuple;

#[derive(Clone, Copy, Debug)]
pub struct PointStruct {}
pub type Point = Tuple<PointStruct, 4>;

impl std::ops::Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl Into<Vector> for Point {
    fn into(self) -> Vector {
        Vector::new(self[0], self[1], self[2])
    }
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point::from([x, y, z, 1.0])
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
