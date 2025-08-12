#![no_std]

use fb::display::FbDisplay;

pub trait Section {
    const STATIC: bool;

    fn render(&self, screen: &mut FbDisplay);
}
