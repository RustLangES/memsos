use crate::mem::write::memwrite;
use crate::ui::{layout::vertical::VerticalLayout, layout::Layout};
use crate::{layout, text};
use bootloader_api::info::{MemoryRegion, MemoryRegionKind};

pub fn run_test(debug_layout: &VerticalLayout, region: MemoryRegion) {
    if region.kind != MemoryRegionKind::Usable {
        layout!(
            debug_layout,
            &text!((0, 0), "Omitting region of memory {:?}", region)
        );
        return;
    }

    layout!(debug_layout, &text!((0, 0), "Checking region {:?}", region));

    for addr in region.start..region.end {
        memwrite(addr, 1);
    }
}
