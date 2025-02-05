#![no_std]

use core::fmt::Arguments; 

pub fn run_test<M: Mem, L: Logger>(logger: &L, mem: &M, region: MemoryRegion) {
    // Simulating a test this should be a real test in the future

    logger.log(format_args!("Checking region {:?}", region));
    let offset_region = mem.parse(region);

    for addr in offset_region.start..offset_region.end {
        if !mem.check(addr) {
            continue;
        }        

        mem.write(addr, 2);

        if mem.read(addr) != 2 {
            panic!("Oh no!");
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
    fn check(&self, addr: u64) -> bool;
    fn read(&self, addr: u64) -> u64;
    fn write(&self, addr: u64, value: u64);
}

pub trait Logger {
    fn log(&self, message: Arguments<'_>);
}

