use embedded_graphics::{
    Drawable,
    mono_font::{MonoFont, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::{DrawTarget, Point, RgbColor, Size},
    primitives::Rectangle,
    text::Text,
};

use crate::components::Component;

pub struct UiText {
    pub text: &'static str,
    pub color: Rgb888,
    pub font: MonoFont<'static>,
    pub pos: Point,
}

impl Component for UiText {
    fn render(
        &self,
        screen: &mut fb::display::FbDisplay,
        space: embedded_graphics::prelude::Point,
    ) {
        let style = MonoTextStyle::new(&self.font, self.color);
        let text = Text::new(self.text, self.pos + space, style);

        text.draw(screen).unwrap();
    }
    fn clear(&self, screen: &mut fb::display::FbDisplay, space: embedded_graphics::prelude::Point) {
        screen
            .fill_solid(
                &Rectangle::new(
                    Point {
                        x: self.pos.x + space.x,
                        y: (self.pos.y + space.y - (self.font.character_size.height as i32) / 2)
                            - 2,
                    },
                    Size::new(
                        self.font.character_size.width * (self.text.len() as u32),
                        self.font.character_size.height,
                    ),
                ),
                Rgb888::BLACK,
            )
            .unwrap();
    }
}
