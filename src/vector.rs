use crate::{
    point::Point,
    tuple::{Tuple, TupleAdd, TupleSub},
};

#[derive(Clone, Copy, Debug)]
pub struct VectTuple {}

// Vectors can add and subtract
impl TupleAdd for VectTuple {}
impl TupleSub for VectTuple {}

pub type Vector = Tuple<VectTuple, 4>;

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector::from([x, y, z, 0.0])
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2] + self[3] * other[3]
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Vector::new(
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        )
    }

    pub fn magnitude(&self) -> f64 {
        (self[0] * self[0] + self[1] * self[1] + self[2] * self[2]).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        let mag = self.magnitude();
        Vector::new(self[0], self[1], self[2]) / mag
    }

    pub fn reflect(&self, normal: Vector) -> Vector {
        *self - normal * 2.0 * self.dot(&normal)
    }
}

impl From<Point> for Vector {
    fn from(p: Point) -> Vector {
        Vector::new(p[0], p[1], p[2])
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_1_SQRT_2;

    use super::*;
    use crate::assert_fuzzy_eq;
    use crate::utils::FuzzyEq;

    macro_rules! dot_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input1, input2, expected) = $value;
                    assert_eq!(input1.dot(&input2), expected);
                }
            )*
        };
    }

    dot_tests!(
        dot_normal_vecs: (Vector::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0), 0.0),
        dot_more_normal_vecs: (Vector::new(0.0, 0.0, 1.0), Vector::new(0.0, 1.0, 0.0), 0.0),
        dot_non_normal: (Vector::new(1.0, 2.0, 3.0), Vector::new(2.0, 3.0, 4.0), 20.0),
    );

    macro_rules! cross_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input1, input2, expected) = $value;
                    assert_fuzzy_eq!(input1.cross(&input2), expected);
                    assert_fuzzy_eq!(input2.cross(&input1), -expected);
                }
            )*
        };
    }

    cross_tests!(
        cross_1: (Vector::new(1.0, 2.0, 3.0), Vector::new(2.0, 3.0, 4.0), Vector::new(-1.0, 2.0, -1.0)),
        cross_2: (Vector::new(3.0, -2.0, 10.0), Vector::new(0.5, 1.5, 6.0), Vector::new(-27.0, -13.0, 5.5)),
    );

    macro_rules! magnitude_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $value;
                    assert_eq!(input.magnitude(), expected);
                }
            )*
        };
    }

    magnitude_tests!(
        mag_unit_vec: (Vector::new(1.0, 0.0, 0.0), 1.0),
        mag_perfect_square: (Vector::new(3.0, 4.0, 12.0), 13.0),
    );

    macro_rules! normalize_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $value;
                    assert_fuzzy_eq!(input.normalize(), expected);
                    assert_fuzzy_eq!(input.normalize().magnitude(), 1.0);
                }
            )*
        };
    }

    normalize_tests!(
        norm_unit_vec1: (Vector::new(1.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0)),
        norm_unit_vec2: (Vector::new(0.0, 1.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
        norm_unit_vec3: (Vector::new(0.0, 0.0, 1.0), Vector::new(0.0, 0.0, 1.0)),
        norm_perfect_square: (Vector::new(3.0, 4.0, 12.0), Vector::new(3.0 / 13.0, 4.0 / 13.0, 12.0 / 13.0)),
    );

    #[test]
    fn reflecting_a_vector_at_45_degrees() {
        let v = Vector::new(1.0, -1.0, 0.0);
        let n = Vector::new(0.0, 1.0, 0.0);
        let r = v.reflect(n);

        assert_fuzzy_eq!(Vector::new(1.0, 1.0, 0.0), r)
    }

    #[test]
    fn reflecting_a_vector_of_a_slanted_surface() {
        let v = Vector::new(0.0, -1.0, 0.0);
        let n = Vector::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0);
        let r = v.reflect(n);

        assert_fuzzy_eq!(Vector::new(1.0, 0.0, 0.0), r);
    }
}
