use embedded_graphics::{pixelcolor::Rgb888, prelude::Point};

use crate::components::{Component, placeholder::PlaceholderText};

pub struct RawText<const N: usize> {
    pub data: [u8; N],
    pub pos: Point,
    pub color: Rgb888,
}

impl<const N: usize> Component for RawText<{ N }> {
    fn clear(&self, _screen: &mut fb::display::FbDisplay, _space: Point) {
        todo!();
    }
    fn render(&mut self, _screen: &mut fb::display::FbDisplay, space: Point) {
        let placeholder = PlaceholderText::new(self.pos + space);

        placeholder.render_text(
            str::from_utf8(&self.data[..]).unwrap_or("Invalid oem id"),
            self.color,
        );
    }
}
