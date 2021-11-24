use crate::{matrix::Matrix, point::Point, vector::Vector};

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn position(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub fn transform(&self, m: Matrix<4>) -> Self {
        Self {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_fuzzy_eq;
    use crate::utils::*;

    use super::*;

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = Point::new(1.0, 2.0, 3.0);
        let direction = Vector::new(4.0, 5.0, 6.0);

        let ray = Ray::new(origin, direction);

        assert_fuzzy_eq!(ray.origin, origin);
        assert_fuzzy_eq!(ray.direction, direction);
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let ray = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));

        assert_fuzzy_eq!(ray.position(0.0), Point::new(2.0, 3.0, 4.0));
        assert_fuzzy_eq!(ray.position(1.0), Point::new(3.0, 3.0, 4.0));
        assert_fuzzy_eq!(ray.position(-1.0), Point::new(1.0, 3.0, 4.0));
        assert_fuzzy_eq!(ray.position(2.5), Point::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn translating_a_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = Matrix::translate(3.0, 4.0, 5.0);
        let r2 = r.transform(m);

        assert_fuzzy_eq!(Point::new(4.0, 6.0, 8.0), r2.origin);
        assert_fuzzy_eq!(Vector::new(0.0, 1.0, 0.0), r2.direction);
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = Matrix::scale(2.0, 3.0, 4.0);
        let r2 = r.transform(m);

        assert_fuzzy_eq!(Point::new(2.0, 6.0, 12.0), r2.origin);
        assert_fuzzy_eq!(Vector::new(0.0, 3.0, 0.0), r2.direction);
    }
}
