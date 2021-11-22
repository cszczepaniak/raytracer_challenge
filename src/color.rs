use crate::tuple::{ElementwiseMul, Tuple, TupleSub};

#[derive(Clone, Copy, Debug)]
pub struct ColorTuple {}

// Colors can subtract and elementwise multiply
impl TupleSub for ColorTuple {}
impl ElementwiseMul for ColorTuple {}

pub type Color = Tuple<ColorTuple, 3>;

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color::from([r, g, b])
    }

    pub fn clamp(&self, lower: f64, upper: f64) -> Self {
        Color::new(
            self[0].clamp(lower, upper),
            self[1].clamp(lower, upper),
            self[2].clamp(lower, upper),
        )
    }
}
