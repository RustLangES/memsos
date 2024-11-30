use core::marker::PhantomData;

use embedded_graphics::pixelcolor::raw::ToBytes;
use embedded_graphics::prelude::{Dimensions, DrawTarget, PixelColor, Point, Size};
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::Pixel;

pub struct MemsosUIWriter<C: PixelColor> {
    width: u32,
    height: u32,
    stride: u32,
    bytes_per_pixel: u32,
    pixels: &'static mut [u8],
    area: Rectangle,
    _color: PhantomData<C>,
}

impl<C> MemsosUIWriter<C>
where
    C: PixelColor + ToBytes,
    <C as ToBytes>::Bytes: AsRef<[u8]>,
{
    pub fn new(
        width: u32,
        height: u32,
        stride: u32,
        bytes_per_pixel: u32,
        pixels: &'static mut [u8],
    ) -> Self {
        let area = Rectangle::new(Point::zero(), Size::new(width, height));
        Self {
            area,
            width,
            pixels,
            height,
            stride,
            bytes_per_pixel,
            _color: PhantomData::default(),
        }
    }

    fn point_to_index(&self, point: Point) -> Option<usize> {
        if let Ok((x, y)) = <(u32, u32)>::try_from(point) {
            if x < self.width && y < self.height {
                return Some((y * self.stride + x) as usize);
            }
        }

        None
    }
}

impl<C> Dimensions for MemsosUIWriter<C>
where
    C: PixelColor + ToBytes,
    <C as ToBytes>::Bytes: AsRef<[u8]>,
{
    fn bounding_box(&self) -> Rectangle {
        self.area
    }
}

impl<C> DrawTarget for MemsosUIWriter<C>
where
    C: PixelColor + ToBytes,
    <C as ToBytes>::Bytes: AsRef<[u8]>,
{
    type Color = C;
    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels.into_iter() {
            if let Some(index) = self.point_to_index(point) {
                self.pixels[index..index + self.bytes_per_pixel as usize]
                    .copy_from_slice(color.to_be_bytes().as_ref());
            }
        }
        Ok(())
    }
}
