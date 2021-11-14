use super::vector::Vector;
use crate::tuple_type;

tuple_type!(Point, 4, (sub => output = Vector));

impl Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Point {
            data: [x, y, z, 1.0],
        }
    }
}
