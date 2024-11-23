use crate::println;
use bootloader_api::info::MemoryRegion;
use bootloader_api::info::MemoryRegionKind;
use core::fmt::Write;
use heapless::String;
use heapless::Vec;

const VEC_MAX_SIZE: usize = 1024;

use core::ptr;

pub fn test_memory(region: &MemoryRegion) -> bool {
    if region.kind != MemoryRegionKind::Usable {
        return true;
    }

    let pattern: u8 = 0xFF;
    let mut memory_regions_tested: Vec<u16, VEC_MAX_SIZE> = Vec::new();

    println!(
        "Checking memory from 0x{:x} to 0x{:x}",
        region.start, region.end
    );

    unsafe {
        let max_address: u64 = 0xFFFF_FFFF_FFFF_FFFF;
        for addr in region.start..region.end {
            let ptr = addr as *const u8;
            let value = ptr.read_volatile();
        }
    }
    false
}

fn is_memory_address_safe(region: &MemoryRegion, addr: u64) -> bool {
    addr >= region.start && addr < region.end && region.kind == MemoryRegionKind::Usable
}

fn is_aligned(addr: u64, alignment: u64) -> bool {
    addr % alignment == 0
}
