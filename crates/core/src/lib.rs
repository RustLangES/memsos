#![no_std]

use core::fmt::Arguments;
use core::result::Result;

pub fn run_test<M: Mem, L: Logger>(logger: &L, mem: &M, region: MemoryRegion) {
    // Simulating a test this should be a real test in the future

    logger.log(format_args!("Checking region {:?}", region));
    let offset_region = mem.parse(region);
    for addr in offset_region.start..offset_region.end {
        // TODO: move to Mem trait
        let ptr = addr as *mut u64;


        unsafe {
            *ptr = 2;
        
            if ptr.read() != 2 {
                panic!();
            }
        }
    }
}

#[derive(Debug)]
pub struct MemoryRegion {
    pub start: u64,
    pub end: u64,
}

pub trait Mem {
    fn parse(&self, region: MemoryRegion) -> MemoryRegion;
    fn read(&self, addr: u64) -> Result<u64, MemError>;
    fn write(&self, addr: u64, value: u64) -> Result<(), MemError>;
}

pub trait Logger {
    fn log(&self, message: Arguments<'_>);
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum MemError {
    NullPtr,
    MisalignedPtr(u64),
    NoMoreMemory,
}
