use crate::{color::Color, point::Point};

pub struct PointLight {
    pub position: Point,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> Self {
        PointLight {
            position,
            intensity,
        }
    }
}
