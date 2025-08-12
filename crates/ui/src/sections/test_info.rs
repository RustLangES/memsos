use embedded_graphics::{
    Drawable,
    mono_font::{MonoTextStyle, ascii::FONT_6X10},
    pixelcolor::Rgb888,
    prelude::{Point, RgbColor},
    text::Text,
};

use crate::Section;

pub struct TestInfoSection {
    pub current_test: &'static str,
    pub point: Point,
}

impl Section for TestInfoSection {
    fn render(&self, screen: &mut fb::display::FbDisplay) {
        let text = {
            let p = Point::new(0, 0) + self.point;
            let style = MonoTextStyle::new(&FONT_6X10, Rgb888::WHITE);

            Text::new(self.current_test, p, style)
        };

        text.draw(screen).unwrap();
    }
    fn clear(&self, screen: &mut fb::display::FbDisplay) {}
}
