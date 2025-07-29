#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use boot::HIGHER_HALF_OFFSET;
use commons::mem::{is_usable_memory, MemModule, MemoryError, MemoryMap, MemoryReport};
use core::{fmt::Write, ptr::write_bytes};
use fb::println;
use limine::memory_map::{Entry, EntryType};

pub struct MarchC {
    mem_map: MemoryMap,
}

impl MemModule for MarchC {
    const NAME: &'static str = "March C";

    fn init(memory_map: MemoryMap) -> Self {
        Self {
            mem_map: memory_map,
        }
    }

    fn run(&mut self, reports: &mut Vec<MemoryReport>) {
        for entry in self.mem_map {
            if is_usable_memory(entry) {
                run_march_c(reports, &entry);
            }
        }
    }
}

/// # Panics
///
/// It can panics if `entry.length` can't be a usize
#[inline]
fn run_march_c(reports: &mut Vec<MemoryReport>, entry: &Entry) {
    let start = entry.base + *HIGHER_HALF_OFFSET;
    let end = start + entry.length;
    let mut errors = Vec::new();

    unsafe {
        write_bytes(
            start as *mut u8,
            0,
            usize::try_from(entry.length).expect("Len is invalid"),
        );
    }

    for addr in start..end {
        let ptr = addr as *mut u8;
        unsafe {
            if ptr.read_volatile() != 0 && !errors.contains(&ptr.addr()) {
                errors.push(ptr.addr());
                reports.push(MemoryReport {
                    address: ptr.addr(),
                    kind: MemoryError::StuckAt,
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
                    kind: MemoryError::StuckAt,
                });
            }

            ptr.write_volatile(0);
        }
    }
}
