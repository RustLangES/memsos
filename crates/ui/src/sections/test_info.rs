use crate::Section;

pub struct TestInfo {}

impl Section for TestInfo {
    fn render(
        &self,
        screen: &mut fb::display::FbDisplay,
        point: embedded_graphics::prelude::Point,
    ) {
        todo!();
    }
}
