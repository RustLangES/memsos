use arch::tsc::Instant;
use core::fmt::Write;
use embedded_graphics::prelude::Point;
use fb::{get_fb_writer, println};

use crate::components::Component;

pub struct UiTime {
    pub instant: Instant,
    pub pos: Point,
    pub enabled: bool,
}

impl UiTime {
    pub fn new(pos: Point) -> Self {
        Self {
            instant: Instant::now(),
            enabled: true,
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
        println!("{e}");
    }
    fn clear(&self, screen: &mut fb::display::FbDisplay, space: embedded_graphics::prelude::Point) {
    }
}
