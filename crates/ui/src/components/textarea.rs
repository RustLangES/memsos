use embedded_graphics::prelude::Point;

use crate::components::Component;

pub struct UiTextArea {
    pub start_pos: Point,
    pub end_pos: Point,
}

impl Component for UiTextArea {
    type ExtraArgs = &'static str;

    fn render(
        &mut self,
        screen: &mut fb::display::FbDisplay,
        space: Point,
        extra: Self::ExtraArgs,
    ) {
        todo!();
    }
    fn clear(&self, screen: &mut fb::display::FbDisplay, space: Point) {
        todo!();
    }
}
