use png::EncodingError;

use super::{to_rgba32::ToRGBA32, Rectangle};

pub trait ToPNG {
    fn to_png(&self) -> Result<Vec<u8>, EncodingError>;
}

impl<T> ToPNG for T
where
    T: ToRGBA32 + Rectangle,
{
    fn to_png(&self) -> Result<Vec<u8>, EncodingError> {
        let mut data = Vec::new();
        let mut encoder = png::Encoder::new(&mut data, self.width() as u32, self.height() as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.to_rgba32())?;
        writer.finish()?;

        Ok(data)
    }
}
