use crate::tuple::{ElementwiseMul, GenericTuple, TupleSub};

#[derive(Clone, Copy, Debug)]
pub struct ColorTuple {}

// Colors can subtract and elementwise multiply
impl TupleSub for ColorTuple {}
impl ElementwiseMul for ColorTuple {}

pub type Color = GenericTuple<f64, ColorTuple, 3>;

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color::from([r, g, b])
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
