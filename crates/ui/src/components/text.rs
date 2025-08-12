use embedded_graphics::{
    Drawable,
    mono_font::{MonoFont, MonoTextStyle},
    pixelcolor::Rgb888,
    text::Text,
};

use crate::components::Component;

pub struct UiText {
    pub text: &'static str,
    pub color: Rgb888,
    pub font: MonoFont<'static>,
}

impl Component for UiText {
    fn render(&self, screen: &mut fb::display::FbDisplay, pos: embedded_graphics::prelude::Point) {
        let style = MonoTextStyle::new(&self.font, self.color);
        let text = Text::new(self.text, pos, style);

        text.draw(screen).unwrap();
    }
}
