#![no_std]

use core::fmt::Arguments;
use core::result::Result;

pub fn run_test<M: Mem, L: Logger>(logger: &L, mem: &M, region: MemoryRegion) {
    logger.log(format_args!("Welcome in the memsos {}!", "core"));
}

pub struct MemoryRegion {}

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
    MisalignedPtr,
    NoMoreMemory
}
