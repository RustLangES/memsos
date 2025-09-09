use core::fmt::Write;
use embedded_graphics::{pixelcolor::Rgb888, prelude::Point};
use fb::{color_print, get_fb_writer};
use timers::tsc::Instant;

use crate::components::Component;

pub struct UiTime {
    pub instant: Instant,
    pub pos: Point,
    pub enabled: bool,
    pub text_color: Rgb888,
}

impl UiTime {
    pub fn new(pos: Point, text_color: Rgb888) -> Self {
        Self {
            instant: Instant::now(),
            enabled: true,
            text_color,
            pos,
        }
    }
}

impl Component for UiTime {
    fn render(
        &mut self,
        _screen: &mut fb::display::FbDisplay,
        space: embedded_graphics::prelude::Point,
    ) {
        let e = self.instant.to_timestamp();
        let p = self.pos + space;
        let fb = get_fb_writer();
        fb.x = p.x as usize;
        fb.y = p.y as usize;
        color_print!(self.text_color, "{e}");
    }
    fn clear(
        &self,
        _screen: &mut fb::display::FbDisplay,
        _space: embedded_graphics::prelude::Point,
    ) {
    }
}
