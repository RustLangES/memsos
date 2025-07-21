#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use fb::println;
use core::{fmt::Write, ptr::write_bytes};
use boot::HIGHER_HALF_OFFSET;
use commons::mem::{MemModule, MemoryError, MemoryMap, MemoryReport};
use limine::memory_map::{Entry, EntryType};


pub struct MarchC {
    mem_map: MemoryMap
}

impl MemModule for MarchC {
    const NAME: &'static str = "March C";
    
    fn init(memory_map: MemoryMap) -> Self {
        Self { mem_map: memory_map } 
    }

    fn run(&mut self, reports: &mut Vec<MemoryReport>) {
        for entry in self.mem_map {
            if entry.entry_type == EntryType::USABLE {
                run_march_c(reports, &entry);
            }
        }
    }
}

#[inline]
fn run_march_c(reports: &mut Vec<MemoryReport>, entry: &Entry) {
    let start = entry.base + *HIGHER_HALF_OFFSET;
    let end = start + entry.length;
    let mut errors = Vec::new();

    unsafe {
        write_bytes(start as *mut u8, 0, entry.length as usize);
    }


    for addr in start..end {
        let ptr = addr as *mut u8;
        unsafe {
            if ptr.read_volatile() != 0 && !errors.contains(&ptr.addr()) {
                errors.push(ptr.addr());
                reports.push(MemoryReport {
                    address: ptr.addr(),
                    kind: MemoryError::StuckAt
                });
            }

            ptr.write_volatile(1);
        }
    }

    for addr in (start..end).rev() {
        let ptr = addr as *mut u8;
        unsafe {
            if ptr.read_volatile() != 1 && !errors.contains(&ptr.addr()) {
                errors.push(ptr.addr());
                reports.push(MemoryReport {
                    address: ptr.addr(),
                    kind: MemoryError::StuckAt
                });
            }

            ptr.write_volatile(0);
        }

    }
}
