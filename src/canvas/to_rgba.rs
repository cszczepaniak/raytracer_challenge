use super::Canvas;

pub trait ToRgba {
    fn to_rgba(&self) -> Vec<u8>;
}

impl ToRgba for Canvas {
    fn to_rgba(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        for pixel in self.pixels.iter() {
            let clamped = pixel.clamp(0.0, 1.0);
            let r = (clamped[0] * 255.0).round() as u8;
            let g = (clamped[1] * 255.0).round() as u8;
            let b = (clamped[2] * 255.0).round() as u8;
            data.push(r);
            data.push(g);
            data.push(b);
            data.push(255); // alpha channel
        }
        data
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;

    use super::*;

    #[test]
    fn to_rgba_works_for_canvas() {
        let mut c = Canvas::new(2, 2);
        c.write_pixel(0, 0, Color::new(1.0, 0.0, 0.0));
        c.write_pixel(1, 0, Color::new(0.0, 1.0, 0.0));
        c.write_pixel(0, 1, Color::new(0.0, 0.0, 1.0));
        c.write_pixel(1, 1, Color::new(0.5, 1.5, -1.0));

        assert_eq!(
            vec![255u8, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 128, 255, 0, 255],
            c.to_rgba()
        )
    }
}
