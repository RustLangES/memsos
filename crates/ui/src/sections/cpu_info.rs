use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{Point, RgbColor},
};

use crate::{
    Section,
    components::{Component, raw_text::RawText},
};

pub struct CpuInfoSection {
    pub ui_text: RawText<6>,
    pub pos: Point,
}

impl CpuInfoSection {
    pub fn new(oem_id: [u8; 6], pos: Point) -> Self {
        CpuInfoSection {
            ui_text: RawText {
                data: oem_id,
                pos: Point::new(20, 20),
                color: Rgb888::WHITE,
            },
            pos,
        }
    }
}

impl Section for CpuInfoSection {
    fn render(&mut self, screen: &mut fb::display::FbDisplay) {
        self.ui_text.render(screen, self.pos);
    }
}
