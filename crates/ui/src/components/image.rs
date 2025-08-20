use embedded_graphics::{
    Drawable,
    image::Image,
    pixelcolor::Rgb888,
    prelude::{DrawTarget, OriginDimensions, Point, RgbColor},
    primitives::Rectangle,
};
use tinytga::Tga;

use crate::components::Component;

pub struct UiImage {
    pub content: &'static [u8],
    pub pos: Point,
}

impl UiImage {
    pub fn new(content: &'static [u8], pos: Point) -> Self {
        let tga: Tga<Rgb888> = Tga::from_slice(content).unwrap();
        let size = tga.size();

        Self {
            content,
            pos: Point::new(pos.x - size.width as i32, pos.y - size.height as i32),
        }
    }
}

impl Component for UiImage {
    fn render(&mut self, screen: &mut fb::display::FbDisplay, space: Point) {
        let tga: Tga<Rgb888> = Tga::from_slice(self.content).unwrap();

        let image = Image::new(&tga, self.pos + space);

        image.draw(screen).unwrap();
    }
    fn clear(&self, screen: &mut fb::display::FbDisplay, space: Point) {
        let tga: Tga<Rgb888> = Tga::from_slice(self.content).unwrap();
        let size = tga.size();

        screen
            .fill_solid(&Rectangle::new(self.pos + space, size), Rgb888::BLACK)
            .unwrap();
    }
}
