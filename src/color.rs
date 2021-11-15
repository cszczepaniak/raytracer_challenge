use crate::tuple::{ElementwiseMul, Tuple, TupleSub};

#[derive(Clone, Copy, Debug)]
pub struct ColorTuple {}

// Colors can subtract and elementwise multiply
impl TupleSub for ColorTuple {}
impl ElementwiseMul for ColorTuple {}

pub type Color = Tuple<f64, ColorTuple, 3>;

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color::from([r, g, b])
    }

    pub fn to_bytes(&self) -> (u8, u8, u8) {
        let scaled = self.clone() * 255.0;
        (
            scaled[0].round() as u8,
            scaled[1].round() as u8,
            scaled[2].round() as u8,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_bytes_saturates() {
        let c = Color::new(-1.0, 100000.0, 0.5);
        let (r, g, b) = c.to_bytes();
        assert_eq!(0, r);
        assert_eq!(255, g);
        assert_eq!(128, b);
    }

    #[test]
    fn to_bytes_rounds() {
        let c = Color::new(0.1, 0.099999999, 0.499999);
        let (r, g, b) = c.to_bytes();
        assert_eq!(26, r);
        assert_eq!(25, g);
        assert_eq!(127, b);
    }
}
