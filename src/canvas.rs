use crate::color::Color;

mod to_png;
mod to_ppm;
mod to_rgba;

pub use to_png::*;
pub use to_ppm::*;
pub use to_rgba::*;

pub trait Rectangle {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

pub struct Canvas {
    pub width: usize,
    pub height: usize,

    pixels: Vec<Color>,
}

impl Rectangle for Canvas {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![Color::default(); width * height],
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        let idx = self.pixel_index_at(x, y);
        self.pixels[idx] = c;
    }

    pub fn read_pixel(&self, x: usize, y: usize) -> Color {
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
    use crate::fuzzy_eq::FuzzyEq;

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
