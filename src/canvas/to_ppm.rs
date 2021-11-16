use super::{to_rgba::ToRGBA, Rectangle};

pub trait ToPPM {
    fn ppm_header(&self) -> Vec<u8>
    where
        Self: Rectangle,
    {
        format!("P3\n{} {}\n{}\n", self.width(), self.height(), 255).into()
    }

    fn to_ppm(&self) -> Vec<u8>;
}

impl<T> ToPPM for T
where
    T: ToRGBA + Rectangle,
{
    fn to_ppm(&self) -> Vec<u8> {
        let mut res = Vec::from(self.ppm_header());
        let mut pixels_written = 0usize;
        let mut row_width = 0usize;
        for bytes in self.to_rgba().chunks(4) {
            // skip the alpha value
            for b in bytes.iter().take(3) {
                let comp = format!("{}", b);
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
            if pixels_written == self.width() {
                // wrap after we write a width's worth of pixels
                res.extend(b"\n");
                pixels_written = 0;
                row_width = 0;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canvas::Canvas;
    use crate::color::Color;

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
        assert_eq!(exp, c.to_ppm());
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
        let mut c = Canvas::new(24, 2);
        c.write_pixel(0, 0, Color::new(0.0, 0.0, 0.0));

        let exp_header = String::from("P3\n24 2\n255\n");
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
