use core::fmt::Write;
use embedded_graphics::prelude::Point;
use fb::{get_fb_writer, println};

pub struct PlaceholderText {
    pub pos: Point,
}

impl PlaceholderText {
    pub fn new(pos: Point) -> Self {
        Self { pos }
    }
    pub fn render_text(&self, text: &'static str) {
        let fb = get_fb_writer();
        fb.y = self.pos.x as usize;
        fb.x = self.pos.y as usize;

        println!("{}", text);
    }
}
