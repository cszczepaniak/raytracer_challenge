use std::{marker::PhantomData, ops};

use super::tuple::Tuple;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VectTuple;

pub type Vector = Tuple<VectTuple>;

// Subtracting two vectors makes a vector
impl ops::Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Tuple(x, y, z, 0.0, PhantomData)
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2 + self.3 * other.3
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Vector::new(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn magnitude(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        *self / self.magnitude()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_vecs() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, -4.0);
        assert_eq!(v1 - v2, Vector::new(-1.0, -1.0, 7.0));
    }

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
                    assert_eq!(input1.cross(&input2), expected);
                    assert_eq!(input2.cross(&input1), -expected);
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
                    assert_eq!(input.normalize(), expected);
                    assert_eq!(input.normalize().magnitude(), 1.0);
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
}
