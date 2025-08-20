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
        Self { content, pos }
    }
}

impl Component for UiImage {
    fn render(&mut self, screen: &mut fb::display::FbDisplay, space: Point) {
        let tga: Tga<Rgb888> = Tga::from_slice(self.content).unwrap();
        let size = tga.size();
        let image = Image::new(
            &tga,
            Point::new(
                (size.width - self.pos.x as u32 + space.x as u32) as i32 / 2,
                (size.height - self.pos.y as u32 + space.y as u32) as i32 / 2,
            ),
        );

        image.draw(screen).unwrap();
    }
    fn clear(&self, screen: &mut fb::display::FbDisplay, space: Point) {
        let tga: Tga<Rgb888> = Tga::from_slice(self.content).unwrap();
        let size = tga.size();

        screen
            .fill_solid(
                &Rectangle::new(
                    Point::new(
                        (size.width - self.pos.x as u32 + space.x as u32) as i32 / 2,
                        (size.height - self.pos.y as u32 + space.y as u32) as i32 / 2,
                    ),
                    size,
                ),
                Rgb888::BLACK,
            )
            .unwrap();
    }
}
