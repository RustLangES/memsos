use core::fmt::Write;
use embedded_graphics::{pixelcolor::Rgb888, prelude::Point};
use fb::{color_print, get_fb_writer};

pub struct PlaceholderText {
    pub pos: Point,
}

impl PlaceholderText {
    pub fn new(pos: Point) -> Self {
        Self { pos }
    }
    pub fn render_text(&self, text: &str, color: Rgb888) {
        let fb = get_fb_writer();
        fb.y = self.pos.y as usize;
        fb.x = self.pos.x as usize;

        color_print!(color, "{}", text);
    }
}
