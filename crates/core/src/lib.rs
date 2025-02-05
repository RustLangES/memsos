#![no_std]

use core::fmt::Arguments; 

pub fn run_test<M: Mem, L: Logger>(logger: &L, mem: &M, region: MemoryRegion) {
    logger.log(format_args!("Checking region {:?}", region));
    let offset_region = mem.parse(region);

    for addr in offset_region.start..offset_region.end {
        if !mem.check(addr) {
            continue;
        }
        mem.write(addr, 0);
    }

    for addr in offset_region.start..offset_region.end {
        if !mem.check(addr) {
            continue;
        }
        if mem.read(addr) != 0 {
            panic!("Test failed at address 1 {:?}", addr);
        }
        mem.write(addr, 1);
    }

    for addr in (offset_region.start..offset_region.end).rev() {
        if !mem.check(addr) {
            continue;
        }
        if mem.read(addr) != 1 {
            panic!("Test failed at address 2 {:?}", addr);
        }
        mem.write(addr, 0);
    }

    for addr in offset_region.start..offset_region.end {
        if !mem.check(addr) {
            continue;
        }
        if mem.read(addr) != 0 {
            panic!("Test failed at address 3 {:?}", addr);
        }
        mem.write(addr, 1);
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

