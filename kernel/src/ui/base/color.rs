use bootloader_api::info::PixelFormat;
use embedded_graphics::pixelcolor::raw::{RawU8, ToBytes};
use embedded_graphics::prelude::PixelColor;

#[derive(PartialEq, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    format: PixelFormat,
}

impl PixelColor for Color {
    type Raw = RawU8;
}

impl ToBytes for Color {
    type Bytes = [u8; 3];

    fn to_be_bytes(self) -> Self::Bytes {
        match self.format {
            PixelFormat::Rgb => [self.r, self.g, self.b],
            PixelFormat::Bgr => [self.b, self.g, self.r],
            PixelFormat::U8 => [self.r, 0, 0], // Assume red channel only for U8
            PixelFormat::Unknown {
                red_position,
                green_position,
                blue_position,
            } => {
                let mut bytes = [0u8; 3];
                bytes[red_position as usize % 3] = self.r;
                bytes[green_position as usize % 3] = self.g;
                bytes[blue_position as usize % 3] = self.b;
                bytes
            }
            _ => [0, 0, 0],
        }
    }

    fn to_le_bytes(self) -> Self::Bytes {
        // For little-endian, simply use the same logic as big-endian for RGB/BGR
        match self.format {
            PixelFormat::Rgb => [self.r, self.g, self.b],
            PixelFormat::Bgr => [self.b, self.g, self.r],
            PixelFormat::U8 => [self.r, 0, 0],
            PixelFormat::Unknown {
                red_position,
                green_position,
                blue_position,
            } => {
                let mut bytes = [0u8; 3];
                bytes[red_position as usize % 3] = self.r;
                bytes[green_position as usize % 3] = self.g;
                bytes[blue_position as usize % 3] = self.b;
                bytes
            }
            _ => [0, 0, 0],
        }
    }

    fn to_ne_bytes(self) -> Self::Bytes {
        // Use macros to determine the target architecture's endianness
        #[cfg(target_endian = "little")]
        {
            self.to_le_bytes()
        }
        #[cfg(target_endian = "big")]
        {
            self.to_be_bytes()
        }
    }
}

impl Color {
    pub const RGB_BLACK: Color = Color::new(0, 0, 0, PixelFormat::Rgb);
    pub const RGB_GREEN: Color = Color::new(0, 255, 0, PixelFormat::Rgb);
    pub const RGB_YELLOW: Color = Color::new(255, 255, 0, PixelFormat::Rgb);
    pub const RGB_WHITE: Color = Color::new(255, 255, 255, PixelFormat::Rgb);

    pub const BGR_BLACK: Color = Color::new(0, 0, 0, PixelFormat::Bgr);
    pub const BGR_GREEN: Color = Color::new(0, 255, 0, PixelFormat::Bgr);
    pub const BGR_YELLOW: Color = Color::new(255, 255, 0, PixelFormat::Bgr);
    pub const BGR_WHITE: Color = Color::new(255, 255, 255, PixelFormat::Bgr);

    pub const U8_BLACK: Color = Color::new(0, 0, 0, PixelFormat::U8);
    pub const U8_GREEN: Color = Color::new(0, 255, 0, PixelFormat::U8);
    pub const U8_YELLOW: Color = Color::new(255, 255, 0, PixelFormat::U8);
    pub const U8_WHITE: Color = Color::new(255, 255, 255, PixelFormat::U8);

    /// Creates a new color instance with specified RGB values and pixel format.
    pub const fn new(r: u8, g: u8, b: u8, format: PixelFormat) -> Self {
        Self { r, g, b, format }
    }
}
