use crate::mem::read::memread;
use crate::mem::write::memwrite;
use crate::ui::MemsosUI;
use bootloader_api::info::MemoryRegion;
use bootloader_api::info::MemoryRegionKind;

const VEC_MAX_SIZE: usize = 1024;

pub fn test_memory(ui: &mut MemsosUI, region: &MemoryRegion, offset: u64) -> bool {
    if region.kind != MemoryRegionKind::Usable {
        return true;
    }

    let mut passed = true;
    let pattern: u64 = 0xFFFFFF;

    // TODO: implement format to no_std
    (*ui).new_row();
    ui.label("Checking memory from ");
    ui.label("to ");

    for addr in region.start + offset..region.end + offset {
        let ptr = addr as *mut u64;

        if ptr.is_null() || (ptr as usize % core::mem::align_of::<u64>() != 0) {
            continue;
        }

        memwrite(ptr, pattern);

        if memread(ptr) != pattern {
            passed = false;
        }
    }

    passed
}
