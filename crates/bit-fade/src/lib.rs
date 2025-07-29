#![no_std]

extern crate alloc;

use core::{ptr::write_bytes, time::Duration};

use alloc::vec::Vec;
use arch::tsc::sleep;
use boot::HIGHER_HALF_OFFSET;
use commons::mem::{MemModule, MemoryMap};
use limine::memory_map::Entry;

pub struct BitFadeTest {
    mem_map: MemoryMap
}

impl MemModule for BitFadeTest {
    const NAME: &'static str = "Bit Fade";

    fn init(memory_map: MemoryMap) -> Self {
       Self {
            mem_map: memory_map
       } 
    }

    fn run(&mut self, reports: &mut Vec<commons::mem::MemoryReport>) {
        todo!(); 
    }
}

fn run_bit_fade(mem: &Entry, sleep_time: Duration, pattern: u8) {
    let start = mem.base + *HIGHER_HALF_OFFSET;
    let end = start + mem.length;
    let base = start as *mut u8;
    unsafe {
        write_bytes(base, pattern, mem.length as usize);
    }

    sleep(sleep_time);

    for i in  start..end {}
}
