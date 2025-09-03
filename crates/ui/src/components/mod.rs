use embedded_graphics::prelude::Point;
use fb::display::FbDisplay;

pub mod image;
pub mod placeholder;
pub mod raw_text;
pub mod text;
pub mod textarea;
pub mod time;

pub trait Component {
    fn redraw(&mut self, screen: &mut FbDisplay, space: Point) {
        self.clear(screen, space);
        self.render(screen, space);
    }
    fn render(&mut self, screen: &mut FbDisplay, space: Point);
    fn clear(&self, screen: &mut FbDisplay, space: Point);
}
