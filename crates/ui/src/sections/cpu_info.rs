use crate::Section;

pub struct CpuInfoSection {}

impl Section for CpuInfoSection {
    fn render(&mut self, screen: &mut fb::display::FbDisplay) {
        todo!();
    }
}
