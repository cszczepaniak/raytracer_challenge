use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

// Adding two vectors makes another vector
impl ops::Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

// Subtracting two vectors makes a vector
impl ops::Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

// Scalar vector multiplication
impl ops::Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

// Negating a vector makes another vector
impl ops::Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

// Scalar vector division
impl ops::Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z, w: 0.0 }
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Vector::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        *self / self.magnitude()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vecs() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, -4.0);
        assert_eq!(v1 + v2, Vector::new(3.0, 5.0, -1.0));
    }

    #[test]
    fn test_sub_vecs() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, -4.0);
        assert_eq!(v1 - v2, Vector::new(-1.0, -1.0, 7.0));
    }

    #[test]
    fn test_negate_vec() {
        assert_eq!(-Vector::new(1.0, -2.0, 3.0), Vector::new(-1.0, 2.0, -3.0))
    }

    #[test]
    fn test_vec_scalar_mult() {
        assert_eq!(Vector::new(1.0, 2.0, 3.0) * 2.0, Vector::new(2.0, 4.0, 6.0))
    }

    #[test]
    fn test_vec_scalar_div() {
        assert_eq!(Vector::new(1.0, 2.0, 3.0) / 2.0, Vector::new(0.5, 1.0, 1.5))
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
