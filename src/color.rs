use crate::tuple_type;

tuple_type!(Color, 3, add, sub, elementwise_mul);

impl Color<f64> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_fuzzy_eq;
    use crate::utils::FuzzyEq;

    #[test]
    fn test_multiply_colors() {
        let c1 = Color::new(0.8, 1.0, 1.2);
        let c2 = Color::new(0.4, 0.75, 0.5);
        assert_fuzzy_eq!(c1 * c2, Color::new(0.32, 0.75, 0.6))
    }

    #[test]
    fn test_sub_colors() {
        let c1 = Color::new(0.8, 0.75, 1.2);
        let c2 = Color::new(0.4, 1.0, 0.5);
        assert_fuzzy_eq!(c1 - c2, Color::new(0.4, -0.25, 0.7))
    }
}
