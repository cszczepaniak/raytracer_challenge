use std::marker::PhantomData;
use std::ops;

use super::tuple::Tuple;
use super::vector::Vector;

struct PointTuple;
type Point = Tuple<PointTuple>;

// Subtracting two points makes a vector
impl ops::Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Vector {
        Vector::new(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Tuple(x, y, z, 1.0, PhantomData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_points() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let p2 = Point::new(2.0, 3.0, -4.0);
        assert_eq!(p1 - p2, Vector::new(-1.0, -1.0, 7.0));
    }
}
