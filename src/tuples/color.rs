use std::ops::{Index, Mul, Sub};

use crate::utils::f64_fuzzy_eq;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    data: [f64; 3],
}

impl Index<usize> for Color {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0..=2 => &self.data[index],
            _ => panic!("index out of range"),
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Color::new(self[0] - other[0], self[1] - other[1], self[2] - other[2])
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Color::new(self[0] * other[0], self[1] * other[1], self[2] * other[2])
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Color::new(self[0] * other, self[1] * other, self[2] * other)
    }
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { data: [r, g, b] }
    }

    pub fn to_bytes(&self) -> (u8, u8, u8) {
        let scaled = self.clone() * 255.0;
        (
            scaled[0].clamp(0.0, 255.0).round() as u8,
            scaled[1].clamp(0.0, 255.0).round() as u8,
            scaled[2].clamp(0.0, 255.0).round() as u8,
        )
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        f64_fuzzy_eq(self[0], other[0])
            && f64_fuzzy_eq(self[1], other[1])
            && f64_fuzzy_eq(self[2], other[2])
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

    #[test]
    fn test_sub_colors() {
        let c1 = Color::new(0.8, 0.75, 1.2);
        let c2 = Color::new(0.4, 1.0, 0.5);
        assert_eq!(c1 - c2, Color::new(0.4, -0.25, 0.7))
    }
}
