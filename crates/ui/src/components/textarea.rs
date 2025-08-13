use core::fmt::Write;
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{Point, RgbColor},
};
use fb::get_fb_writer;

use crate::{
    components::{Component, placeholder::PlaceholderText},
    get_ui_state,
};

pub struct UiTextArea {
    pub start_pos: Point,
    current_pos: Point,
    pub end_pos: Point,
}

impl UiTextArea {
    pub fn new(start_pos: Point, end_pos: Point) -> Self {
        Self {
            start_pos,
            current_pos: start_pos,
            end_pos,
        }
    }
}

impl Component for UiTextArea {
    fn render(&mut self, _screen: &mut fb::display::FbDisplay, _space: Point) {}
    fn clear(&self, _screen: &mut fb::display::FbDisplay, space: Point) {
        for x in self.start_pos.x..=self.end_pos.x {
            for y in self.start_pos.y..=self.end_pos.y {
                get_fb_writer().write_pixel((space.x + x) as u64, (space.y + y) as u64, 0);
            }
        }
    }
}

impl Write for UiTextArea {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let space = get_ui_state().test_info_section.pos;
        let text = PlaceholderText {
            pos: self.current_pos + space,
        };

        if self.current_pos.y >= self.end_pos.y {
            self.current_pos = self.start_pos;
            //self.clear(screen, space);
            //  return Ok(());
        }
        self.current_pos.y += 15;

        text.render_text(s, Rgb888::WHITE);

        Ok(())
    }
}
