use embedded_graphics::prelude::{OriginDimensions, Point};
use fb::get_ui_writer;
use images::LOGO_CONTENT;

use crate::{
    Section,
    components::{Component, image::UiImage},
};

pub struct LoadingSection {
    pub pos: Point,
    logo: UiImage,
}

impl LoadingSection {
    pub fn new(pos: Point) -> Self {
        let mut logo = UiImage::new(
            LOGO_CONTENT,
            Point::new(
                get_ui_writer().size().width as i32,
                get_ui_writer().size().height as i32,
            ),
        );
        logo.pos.x /= 2;
        logo.pos.y /= 2;

        Self { pos, logo }
    }
}

impl Section for LoadingSection {
    fn render(&mut self, screen: &mut fb::display::FbDisplay) {
        self.logo.render(screen, self.pos);
    }
}
