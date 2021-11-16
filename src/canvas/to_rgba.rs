use super::Canvas;
use num_traits::Float;

pub trait ToRGBA {
    fn to_rgba(&self) -> Vec<u8>;
}

impl<T> ToRGBA for Canvas<T>
where
    T: Float,
{
    fn to_rgba(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        for pixel in self.pixels.iter() {
            let clamped = pixel.clamp(T::zero(), T::one());
            let r = (clamped[0] * T::from(255.0).unwrap())
                .round()
                .to_u8()
                .unwrap();
            let g = (clamped[1] * T::from(255.0).unwrap())
                .round()
                .to_u8()
                .unwrap();
            let b = (clamped[2] * T::from(255.0).unwrap())
                .round()
                .to_u8()
                .unwrap();
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
