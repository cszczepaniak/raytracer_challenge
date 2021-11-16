use super::Canvas;
use num_traits::Float;

pub trait ToRGBA32 {
    fn to_rgba32(&self) -> Vec<u8>;
}

impl<T> ToRGBA32 for Canvas<T>
where
    T: Float,
{
    fn to_rgba32(&self) -> Vec<u8> {
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
