use num_traits::Float;

use crate::color::Color;

pub mod to_png;
pub mod to_ppm;
pub mod to_rgba32;

pub trait Rectangle {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

pub struct Canvas<T>
where
    T: Float,
{
    pub width: usize,
    pub height: usize,

    pixels: Vec<Color<T>>,
}

impl<T> Rectangle for Canvas<T>
where
    T: Float,
{
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl<T> Canvas<T>
where
    T: Float,
{
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![Color::default(); width * height],
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, c: Color<T>) {
        let idx = self.pixel_index_at(x, y);
        self.pixels[idx] = c;
    }

    pub fn read_pixel(&self, x: usize, y: usize) -> Color<T> {
        self.pixels[self.pixel_index_at(x, y)]
    }

    fn pixel_index_at(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_fuzzy_eq;
    use crate::utils::FuzzyEq;

    #[test]
    fn test_create_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for i in 0..10 {
            for j in 0..20 {
                assert_fuzzy_eq!(c.read_pixel(i, j), Color::new(0.0, 0.0, 0.0));
            }
        }
    }

    #[test]
    fn test_write_to_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        let green = Color::new(0.0, 1.0, 0.0);
        let blue = Color::new(0.0, 0.0, 1.0);
        c.write_pixel(5, 5, red);
        c.write_pixel(6, 6, green);
        c.write_pixel(7, 7, blue);

        for i in 0..10 {
            for j in 0..20 {
                let exp_color = match (i, j) {
                    (5, 5) => red,
                    (6, 6) => green,
                    (7, 7) => blue,
                    _ => Color::new(0.0, 0.0, 0.0),
                };
                assert_fuzzy_eq!(c.read_pixel(i, j), exp_color);
            }
        }
    }
}
