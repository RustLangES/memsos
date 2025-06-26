use core::ops::RangeInclusive;
use alloc::vec::Vec;
use uefi::{boot::{memory_map, MemoryType}, mem::memory_map::MemoryMapOwned};
const ALIGNMENT: usize = core::mem::align_of::<u64>();

pub fn check_addr(addr: usize) -> bool {
    let ptr = addr as *mut u64;
    addr as usize % ALIGNMENT == 0 && !ptr.is_null()
}

pub fn get_usable_mem() -> [Option<MemoryMapOwned> ; 4] {
    let runtime_services_code = memory_map(MemoryType::RUNTIME_SERVICES_CODE).ok();
    let runtime_services_data = memory_map(MemoryType::RUNTIME_SERVICES_DATA).ok();
    
    let acpi_reclaim = memory_map(MemoryType::ACPI_RECLAIM).ok();
    let conventional = memory_map(MemoryType::CONVENTIONAL).ok();

    [runtime_services_code, runtime_services_data, acpi_reclaim, conventional]
}

pub fn read(addr: usize) -> usize {
    let ptr = addr as *const u64;

    unsafe { ptr.read() as usize }
}

pub fn write(addr: usize, value: usize) {
    let ptr = addr as *mut u64;

    unsafe { ptr.write(value as u64); }
}
