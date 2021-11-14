use crate::color::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,

    pixels: Vec<Color<f64>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![Color::default(); width * height],
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, c: Color<f64>) {
        let idx = self.pixel_index_at(x, y);
        self.pixels[idx] = c;
    }

    pub fn read_pixel(&self, x: usize, y: usize) -> Color<f64> {
        self.pixels[self.pixel_index_at(x, y)]
    }

    fn pixel_index_at(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn to_ppm(&self) -> Vec<u8> {
        let mut data = self.create_ppm_header();
        data.extend(self.create_ppm_pixel_data());
        data.into()
    }

    fn create_ppm_pixel_data(&self) -> Vec<u8> {
        let mut res = Vec::new();
        let mut pixels_written = 0usize;
        let mut row_width = 0usize;
        for px in self.pixels.iter() {
            let (r, g, b) = px.to_bytes();

            for comp in [format!("{}", r), format!("{}", g), format!("{}", b)] {
                let want_to_write = if row_width == 0 {
                    // at the beginning of a row, we don't write the leading space
                    comp.len()
                } else {
                    // otherwise, we write the leading space
                    1 + comp.len()
                };

                if row_width + want_to_write > 70 {
                    // wrap at 70 characters
                    res.extend(b"\n");
                    row_width = 0;
                }

                if row_width != 0 {
                    res.extend(b" ");
                    row_width += 1;
                }

                res.extend(comp.as_bytes());
                row_width += comp.len();
            }
            pixels_written += 1;
            if pixels_written == self.width {
                // wrap after we write a width's worth of pixels
                res.extend(b"\n");
                pixels_written = 0;
                row_width = 0;
            }
        }
        res
    }

    fn create_ppm_header(&self) -> Vec<u8> {
        format!(
            "P3\n{} {}\n{}\n",
            self.width,
            self.height,
            self.pixels.iter().fold(0u8, |a, &b| {
                let bytes = b.to_bytes();
                a.max(bytes.0.max(bytes.1).max(bytes.2))
            })
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for i in 0..10 {
            for j in 0..20 {
                assert_eq!(c.read_pixel(i, j), Color::new(0.0, 0.0, 0.0));
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
                assert_eq!(c.read_pixel(i, j), exp_color);
            }
        }
    }

    #[test]
    fn test_construct_ppm_header() {
        let mut c = Canvas::new(5, 3);
        c.write_pixel(0, 0, Color::new(1.5, 0.0, 0.0));
        c.write_pixel(2, 1, Color::new(0.0, 0.5, 0.0));
        c.write_pixel(4, 2, Color::new(-0.5, 0.0, 1.0));

        let exp_header = String::from("P3\n5 3\n255\n");
        let exp_pixel_data = String::from(
            "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n"
        );
        let mut exp = exp_header.into_bytes();
        exp.extend(exp_pixel_data.into_bytes());
        assert_eq!(c.to_ppm(), exp);
    }

    #[test]
    fn test_construct_ppm_header_wider_than_70() {
        let mut c = Canvas::new(12, 5);
        // Make the first row obviously overflow the 70 limit
        c.write_pixel(0, 0, Color::new(1.0, 1.0, 0.0));
        c.write_pixel(10, 0, Color::new(1.0, 0.0, 1.0));

        // Make part of the second row exactly reach the 70 limit
        c.write_pixel(0, 1, Color::new(1.0, 1.0, 50.0 / 255.0));

        // Make part of the third row have a part that goes just over the limit
        c.write_pixel(0, 2, Color::new(1.0, 1.0, 1.0));

        // Make the fourth row have a part that goes just below the limit
        c.write_pixel(0, 3, Color::new(1.0, 1.0, 0.0));

        let exp_header = String::from("P3\n12 5\n255\n");
        let exp_pixel_data = String::from(
            "255 255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 0\n255 0 0 0
255 255 50 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0
255 255 255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0
255 255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0
",
        );
        let mut exp = exp_header.into_bytes();
        exp.extend(exp_pixel_data.into_bytes());
        assert_eq!(c.to_ppm(), exp);
    }

    #[test]
    fn test_construct_ppm_header_mroe_splitting() {
        let mut c = Canvas::new(10, 2);
        let color = Color::new(1.0, 0.8, 0.6);
        for x in 0..10 {
            for y in 0..2 {
                c.write_pixel(x, y, color);
            }
        }

        let exp_header = String::from("P3\n10 2\n255\n");
        let exp_pixel_data = String::from(
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n",
        );
        let mut exp = exp_header.into_bytes();
        exp.extend(exp_pixel_data.into_bytes());
        assert_eq!(c.to_ppm(), exp);
    }

    #[test]
    fn test_multi_wrap() {
        let c = Canvas::new(24, 2);

        let exp_header = String::from("P3\n24 2\n0\n");
        let exp_pixel_data = String::from(
            "0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0
",
        );
        let mut exp = exp_header.into_bytes();
        exp.extend(exp_pixel_data.into_bytes());
        assert_eq!(c.to_ppm(), exp);
    }
}
