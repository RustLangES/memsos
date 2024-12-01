use bootloader_api::info::PixelFormat;
use embedded_graphics::pixelcolor::raw::RawU8;
use embedded_graphics::prelude::PixelColor;

#[derive(PartialEq, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl PixelColor for Color {
    type Raw = RawU8;
}

impl Color {
    pub const BLACK: Color = Color::new(0, 0, 0);
    pub const GREEN: Color = Color::new(0, 255, 0);
    pub const YELLOW: Color = Color::new(255, 255, 0);
    pub const WHITE: Color = Color::new(255, 255, 255);

    /// Creates a new color instance with specified RGB values and pixel format.
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

pub trait ToBytes {
    type Bytes;

    fn to_bytes(&self, format: PixelFormat) -> Self::Bytes;
}

impl ToBytes for Color {
    type Bytes = [u8; 4];

    fn to_bytes(&self, format: PixelFormat) -> Self::Bytes {
        match format {
            PixelFormat::Rgb => [self.r, self.g, self.b, 0],
            PixelFormat::Bgr => [self.b, self.g, self.r, 0],
            PixelFormat::U8 => [self.r, 0, 0, 0],
            PixelFormat::Unknown {
                red_position,
                green_position,
                blue_position,
            } => {
                let mut bytes = [0u8; 4];
                bytes[red_position as usize % bytes.len()] = self.r;
                bytes[green_position as usize % bytes.len()] = self.g;
                bytes[blue_position as usize % bytes.len()] = self.b;
                bytes
            }
            _ => [0, 0, 0, 0],
        }
    }
}
