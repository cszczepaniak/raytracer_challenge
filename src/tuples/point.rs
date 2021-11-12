use std::ops;

use super::vector::Vector;

pub struct Point {
    data: [f64; 3],
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { data: [x, y, z] }
    }
}

impl ops::Index<usize> for Point {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0..=2 => &self.data[index],
            _ => panic!("index out of range"),
        }
    }
}

// Subtracting two points makes a vector
impl ops::Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Vector {
        Vector::new(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
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
