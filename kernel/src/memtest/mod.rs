use crate::mem::read::memread;
use crate::mem::write::memwrite;
use bootloader_api::info::MemoryRegion;
use bootloader_api::info::MemoryRegionKind;
use core::fmt::Write;
use heapless::String;

pub fn test_memory(region: &MemoryRegion, offset: u64) -> bool {
    if region.kind != MemoryRegionKind::Usable {
        return true;
    }

    let mut passed = true;
    let pattern: u64 = 0xFFFFFF;

    //println!(
    //    "Checking memory from 0x{:x} to 0x{:x}",
    //    region.start, region.end
    //);

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
