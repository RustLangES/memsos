use embedded_graphics::prelude::Point;
use fb::display::FbDisplay;

pub mod placeholder;
pub mod text;
pub mod time;

pub trait Component {
    fn render(&mut self, screen: &mut FbDisplay, space: Point);
    fn clear(&self, screen: &mut FbDisplay, space: Point);
}
