#![no_std]

pub mod components;
pub mod sections;

use embedded_graphics::prelude::Point;
use fb::{display::FbDisplay, get_ui_writer};

pub trait Section {
    fn render(&self, screen: &mut FbDisplay);
    fn clear(&self, screen: &mut FbDisplay);
}

pub trait RenderSection {
    fn render_section<T: Section>(&mut self, section: T);
}

impl RenderSection for FbDisplay {
    fn render_section<T: Section>(&mut self, section: T) {
        section.render(self);
    }
}

#[inline]
pub fn render_section<T: Section>(section: T) {
    get_ui_writer().render_section(section);
}
