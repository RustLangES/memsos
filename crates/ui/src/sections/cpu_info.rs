use crate::Section;

pub struct CpuInfoSection {}

impl Section for CpuInfoSection {
    fn render(
        &self,
        screen: &mut fb::display::FbDisplay,
        point: embedded_graphics::prelude::Point,
    ) {
        todo!();
    }
}
