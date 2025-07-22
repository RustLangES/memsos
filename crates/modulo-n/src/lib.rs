#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use boot::HIGHER_HALF_OFFSET;
use commons::mem::{MemModule, MemoryError, MemoryMap, MemoryReport};
use limine::memory_map::{Entry, EntryType};

const PATTERN_A: u8 = 0xAA;
const PATTERN_B: u8 = 0xFF;
const N: usize = 4;

pub struct ModuloN {
    mem_map: MemoryMap,
}

impl MemModule for ModuloN {
    const NAME: &'static str = "Modulo N";

    fn init(memory_map: MemoryMap) -> Self {
        Self {
            mem_map: memory_map,
        }
    }
    fn run(&mut self, reports: &mut Vec<MemoryReport>) {
        for entry in self.mem_map {
            if entry.entry_type == EntryType::USABLE {
                for offset in 0..N {
                    run_modulo_n(offset, entry, reports);
                }
            }
        }
    }
}

fn run_modulo_n(offset: usize, entry: &Entry, reports: &mut Vec<MemoryReport>) {
    let base = (entry.base + *HIGHER_HALF_OFFSET) as usize;
    let end = base + (entry.length as usize);
    let start = base + offset;

    for i in start..end {
        if i % N == 0 {
            unsafe {
                let ptr = i as *mut u8;
                core::ptr::write_volatile(ptr, PATTERN_A);
            }
        } else {
            unsafe {
                let ptr = i as *mut u8;
                core::ptr::write_volatile(ptr, PATTERN_B);
            }
        }
    }

    for i in start..end {
        if i % N == 0 {
            unsafe {
                let ptr = i as *mut u8;
                let val = core::ptr::read_volatile(ptr);
                if val != PATTERN_A {
                    reports.push(MemoryReport {
                        address: ptr.addr(),
                        kind: MemoryError::StuckAt,
                    });
                }
            }
        } else {
            unsafe {
                let ptr = i as *mut u8;
                let val = core::ptr::read_volatile(ptr);

                if val != PATTERN_B {
                    reports.push(MemoryReport {
                        address: ptr.addr(),
                        kind: MemoryError::StuckAt,
                    });
                }
            }
        }
    }
}
