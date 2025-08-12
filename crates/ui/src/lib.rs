#![no_std]

pub mod sections;

use embedded_graphics::prelude::Point;
use fb::{display::FbDisplay, get_ui_writer};

pub trait Section {
    fn render(&self, screen: &mut FbDisplay, point: Point);
}

pub trait RenderSection {
    fn render_section<T: Section>(&mut self, section: T, point: Point);
}

impl RenderSection for FbDisplay {
    fn render_section<T: Section>(&mut self, section: T, point: Point) {
        section.render(self, point);
    }
}

#[inline]
pub fn render_section<T: Section>(section: T, point: Point) {
    get_ui_writer().render_section(section, point);
}
