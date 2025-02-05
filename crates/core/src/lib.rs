#![no_std]

mod test;

use core::fmt::Arguments;
use crate::test::marchc;

pub fn run_test<M: Mem, L: Logger>(logger: &L, mem: &M, region: &MemoryRegion) {
    logger.log(format_args!("Checking region {:?}", region));


    marchc::run_march_c(mem, region);
}

#[derive(Debug)]
pub struct MemoryRegion {
    pub start: u64,
    pub end: u64,
}

pub trait Mem {
    fn parse(&self, region: &MemoryRegion) -> MemoryRegion;
    fn check(&self, addr: u64) -> bool;
    fn read(&self, addr: u64) -> u64;
    fn write(&self, addr: u64, value: u64);
}

pub trait Logger {
    fn log(&self, message: Arguments<'_>);
}
