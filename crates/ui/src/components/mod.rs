use embedded_graphics::prelude::Point;
use fb::display::FbDisplay;

pub mod text;

pub trait Component {
    fn render(&self, screen: &mut FbDisplay, pos: Point);
}
