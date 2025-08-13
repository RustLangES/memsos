use core::fmt::Write;
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{Point, RgbColor},
};
use fb::{color_print, get_fb_writer, println};

pub struct PlaceholderText {
    pub pos: Point,
}

impl PlaceholderText {
    pub fn new(pos: Point) -> Self {
        Self { pos }
    }
    pub fn render_text(&self, text: &'static str, color: Rgb888) {
        let fb = get_fb_writer();
        fb.y = self.pos.x as usize;
        fb.x = self.pos.y as usize;

        color_print!(color, "{}", text);
    }
}
