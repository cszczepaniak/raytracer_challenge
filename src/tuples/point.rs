use crate::tuples::vector::Vector;
use std::ops;

#[derive(Debug)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

// Subtracting two points makes a vector
impl ops::Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Vector {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z, w: 1.0 }
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
