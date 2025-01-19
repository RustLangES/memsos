use crate::mem::read::memread;
use crate::mem::write::memwrite;
use bootloader_api::info::MemoryRegion;
use bootloader_api::info::MemoryRegionKind;
use core::fmt::Write;
use heapless::String;

pub fn test_memory(region: &MemoryRegion, offset: u64) -> bool {
   unimplemented!() 
}
