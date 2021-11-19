use std::io::Write;

use png::EncodingError;

use super::{to_rgba::ToRgba, Rectangle};

pub trait ToPng<T>
where
    T: Write,
{
    fn to_png(&self, w: T) -> Result<(), EncodingError>;
}

impl<T, U> ToPng<U> for T
where
    T: ToRgba + Rectangle,
    U: Write,
{
    fn to_png(&self, w: U) -> Result<(), EncodingError> {
        let mut encoder = png::Encoder::new(w, self.width() as u32, self.height() as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.to_rgba())?;
        writer.finish()?;

        Ok(())
    }
}
