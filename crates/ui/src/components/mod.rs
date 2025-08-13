use embedded_graphics::prelude::Point;
use fb::display::FbDisplay;

pub mod placeholder;
pub mod text;
pub mod textarea;
pub mod time;

pub trait Component {
    type ExtraArgs;

    fn redraw(&mut self, screen: &mut FbDisplay, space: Point, extra_args: Self::ExtraArgs) {
        self.clear(screen, space);
        self.render(screen, space, extra_args);
    }
    fn render(&mut self, screen: &mut FbDisplay, space: Point, extra_args: Self::ExtraArgs);
    fn clear(&self, screen: &mut FbDisplay, space: Point);
}
