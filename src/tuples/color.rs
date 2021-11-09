use std::{marker::PhantomData, ops::Mul};

use crate::utils::f64_fuzzy_eq;

use super::tuple::Tuple;

#[derive(Debug)]
pub struct ColorTuple;
pub type Color = Tuple<ColorTuple>;

impl Color {
    fn new(r: f64, g: f64, b: f64) -> Self {
        Tuple(r, g, b, 0.0, PhantomData)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        f64_fuzzy_eq(self.0, other.0)
            && f64_fuzzy_eq(self.1, other.1)
            && f64_fuzzy_eq(self.2, other.2)
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Color::new(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_colors() {
        let c1 = Color::new(0.8, 1.0, 1.2);
        let c2 = Color::new(0.4, 0.75, 0.5);
        assert_eq!(c1 * c2, Color::new(0.32, 0.75, 0.6))
    }
}
