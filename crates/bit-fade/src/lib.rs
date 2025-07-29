#![no_std]

extern crate alloc;

use core::{ptr::write_bytes, time::Duration};

use alloc::vec::Vec;
use arch::tsc::sleep;
use boot::HIGHER_HALF_OFFSET;
use commons::mem::{is_usable_memory, MemModule, MemoryError, MemoryMap, MemoryReport};
use limine::memory_map::{Entry, EntryType};

const PATTERN_A: u8 = 0b000_101_011;
const TIME_A: Duration = Duration::from_secs(90);

const PATTERN_B: u8 = 0b111_100;
const TIME_B: Duration = Duration::from_secs(120);

const PATTERN_C: u8 = 0b000_110_111;
const TIME_C: Duration = Duration::from_secs(190);

pub struct BitFade {
    mem_map: MemoryMap
}

impl MemModule for BitFade {
    const NAME: &'static str = "Bit Fade";

    fn init(memory_map: MemoryMap) -> Self {
       Self {
            mem_map: memory_map
       } 
    }

    fn run(&mut self, reports: &mut Vec<MemoryReport>) {
       for entry in self.mem_map {
            if is_usable_memory(entry) {
                run_bit_fade(entry, TIME_A, PATTERN_A, reports);
                run_bit_fade(entry, TIME_B, PATTERN_B, reports);
                run_bit_fade(entry, TIME_C, PATTERN_C, reports);
            }
       }
    }
}

fn run_bit_fade(mem: &Entry, sleep_time: Duration, pattern: u8, reports: &mut Vec<MemoryReport>) {
    let start = mem.base + *HIGHER_HALF_OFFSET;
    let end = start + mem.length;
    let base = start as *mut u8;
    unsafe {
        write_bytes(base, pattern, mem.length as usize);
    }

    sleep(sleep_time);

    for i in  start..end {
        let ptr = i as *const u8;

        unsafe {
            let val = ptr.read_volatile();

            if val != pattern {
                reports.push(MemoryReport {
                    address: ptr.addr(),
                    kind: MemoryError::BitFade(sleep_time),
                });
            }
        }
    }
}
