use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{Point, RgbColor},
};
use fb::get_ui_writer;

use crate::{
    Section,
    components::{Component, text::UiText, textarea::UiTextArea, time::UiTime},
};

pub struct TestInfoSection {
    pub current_test: UiText,
    pub time: UiTime,
    pub logs: UiTextArea,
    pub pos: Point,
}

impl TestInfoSection {
    pub fn new(pos: Point) -> Self {
        Self {
            current_test: UiText {
                pos: Point { x: 30, y: 30 },
                text: "",
                color: Rgb888::WHITE,
            },
            time: UiTime::new(Point { x: 30, y: 50 }, Rgb888::WHITE),
            logs: UiTextArea::new(Point { x: 100, y: 100 }, Point { x: 400, y: 450 }),
            pos,
        }
    }
    pub fn set_current_test(&mut self, text: &'static str) {
        self.current_test.text = text;
        // : (
        self.current_test.redraw(get_ui_writer(), self.pos);
    }
    pub fn update_time(&mut self) {
        self.time.redraw(get_ui_writer(), self.pos);
    }
}

impl Section for TestInfoSection {
    fn render(&mut self, screen: &mut fb::display::FbDisplay) {
        self.current_test
            .render(screen, self.current_test.pos + self.pos);

        self.time.render(screen, self.pos);
    }
}
