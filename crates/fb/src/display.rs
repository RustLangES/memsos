use core::ptr::write_bytes;

use embedded_graphics::{
    Pixel,
    pixelcolor::Rgb888,
    prelude::{DrawTarget, OriginDimensions, Size},
};
use limine::framebuffer::Framebuffer;

#[derive(Debug)]

pub struct Unsupported(());

impl Unsupported {}

pub struct FbDisplay(Framebuffer<'static>);

impl FbDisplay {
    pub fn new(fb: Framebuffer<'static>) -> Self {
        Self(fb)
    }
}

impl OriginDimensions for FbDisplay {
    fn size(&self) -> embedded_graphics::prelude::Size {
        Size::new(
            self.0.width().try_into().unwrap(),
            self.0.height().try_into().unwrap(),
        )
    }
}

impl DrawTarget for FbDisplay {
    type Color = Rgb888;
    type Error = Unsupported;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels.into_iter() {
            let (x, y) = (point.x as u64, point.y as u64);

            let pixel_offset = y * self.0.pitch() + x * 4;
            let offset =
                usize::try_from(pixel_offset).expect("Cannot convert the pixel offset to usize");
            unsafe {
                #[allow(clippy::cast_ptr_alignment)]
                let buffer = self.0.addr().add(offset).cast::<Rgb888>();

                *buffer = color;
            }
        }
        Ok(())
    }
}
