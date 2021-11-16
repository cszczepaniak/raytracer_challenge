use num_traits::Float;

use crate::tuple::{ElementwiseMul, Tuple, TupleSub};

#[derive(Clone, Copy, Debug)]
pub struct ColorTuple {}

// Colors can subtract and elementwise multiply
impl TupleSub for ColorTuple {}
impl ElementwiseMul for ColorTuple {}

pub type Color<T> = Tuple<T, ColorTuple, 3>;

impl<T> Color<T>
where
    T: Float,
{
    pub fn new(r: T, g: T, b: T) -> Self {
        Color::from([r, g, b])
    }

    pub fn clamp(&self, lower: T, upper: T) -> Self {
        Color::new(
            self[0].min(upper).max(lower),
            self[1].min(upper).max(lower),
            self[2].min(upper).max(lower),
        )
    }
}
