use crate::println;
use bootloader_api::info::MemoryRegion;
use bootloader_api::info::MemoryRegionKind;
use core::fmt::Write;
use heapless::String;
use heapless::Vec;

const VEC_MAX_SIZE: usize = 1024;

pub fn read_from_memory(address: *const u32) -> u32 {
    unsafe {
        assert!(!address.is_null());
        assert!(address as usize % core::mem::align_of::<u32>() == 0);
        core::ptr::read_volatile(address)
    }
}
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
    for addr in (region.start..region.end + 0x1000).step_by(4) {
        if addr == 0 {
            println!("Skipping null address 0x0");
            continue;
        }

        if is_memory_address_safe(region, addr) && is_aligned(addr, 4) {
            //println!("Writing bytes to address: 0x{:x}", addr);
            unsafe {
                let addr_ptr = addr as *mut u32;
                //println!("{}", read_from_memory(addr_ptr));
                // core::ptr::read_volatile(addr as *mut u8);
            }
        } else {
            //println!(
            //    "Skipping unsafe, unaligned, or invalid address: 0x{:x}",
            //    addr
            //);
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
