use super::vector::Vector;
use crate::{float::Float, tuple_type};

tuple_type!(Point, 4, (sub => output = Vector));

impl<T> Point<T>
where
    T: Float,
{
    fn new(x: T, y: T, z: T) -> Self {
        Point {
            data: [x, y, z, T::identity()],
        }
    }
}
