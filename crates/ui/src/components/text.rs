use embedded_graphics::{
    Drawable,
    mono_font::{MonoFont, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::{DrawTarget, Point, RgbColor, Size},
    primitives::Rectangle,
    text::Text,
};
use fb::{CHAR_RASTER_HEIGHT, CHAR_RASTER_WIDTH};

use crate::components::{Component, placeholder::PlaceholderText};

#[derive(Clone)]
pub struct UiText {
    pub text: &'static str,
    pub pos: Point,
    pub color: Rgb888,
}

impl Component for UiText {
    type ExtraArgs = ();

    fn render(
        &mut self,
        screen: &mut fb::display::FbDisplay,
        space: embedded_graphics::prelude::Point,
        extra: Self::ExtraArgs,
    ) {
        let placeholder = PlaceholderText::new(self.pos + space);

        placeholder.render_text(self.text, self.color);
    }
    fn clear(&self, screen: &mut fb::display::FbDisplay, space: embedded_graphics::prelude::Point) {
        screen
            .fill_solid(
                &Rectangle::new(
                    Point {
                        x: self.pos.x + space.x,
                        y: (self.pos.y + space.y - (CHAR_RASTER_HEIGHT.val() as i32) / 2) - 2,
                    },
                    Size::new(
                        (CHAR_RASTER_WIDTH as u32) * (self.text.len() as u32),
                        CHAR_RASTER_HEIGHT.val() as u32,
                    ),
                ),
                Rgb888::BLACK,
            )
            .unwrap();
    }
}
