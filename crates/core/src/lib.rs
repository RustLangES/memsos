#![no_std]

use core::fmt::Arguments;
use core::result::Result;

pub fn run_test<M: Mem, L: Logger>(logger: &L, mem: &M, region: MemoryRegion) {
    // Simulating a test this should be a real test in the future

    logger.log(format_args!("Checking region {:?}", region));
    for _addr in region.start..region.end {
        
    }
}

#[derive(Debug)]
pub struct MemoryRegion {
    pub start: u64,
    pub end: u64
}


pub trait Mem {
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
    NoMoreMemory
}
