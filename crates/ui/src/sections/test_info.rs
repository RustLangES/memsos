use embedded_graphics::{
    Drawable,
    mono_font::{MonoFont, MonoTextStyle, ascii::FONT_6X10},
    pixelcolor::Rgb888,
    prelude::{Point, RgbColor},
    text::Text,
};
use fb::get_ui_writer;

use crate::{
    Section,
    components::{Component, text::UiText, time::UiTime},
};

pub struct TestInfoSection {
    pub current_test: UiText,
    pub time: UiTime,
    pub pos: Point,
}

impl TestInfoSection {
    pub fn new(font: MonoFont<'static>, color: Rgb888, pos: Point) -> Self {
        Self {
            current_test: UiText {
                pos: Point { x: 30, y: 30 },
                text: "",
            },
            time: UiTime::new(Point { x: 30, y: 50 }),
            pos,
        }
    }
    pub fn set_current_test(&mut self, text: &'static str) {
        self.current_test.text = text;
        // : (
        self.current_test.clear(get_ui_writer(), self.pos);
        self.current_test.render(get_ui_writer(), self.pos);
    }
    pub fn update_time(&mut self) {
        self.time.render(get_ui_writer(), self.pos);
    }
}

impl Section for TestInfoSection {
    fn render(&mut self, screen: &mut fb::display::FbDisplay) {
        self.current_test
            .render(screen, self.current_test.pos + self.pos);

        self.time.render(screen, self.pos);
    }
}
