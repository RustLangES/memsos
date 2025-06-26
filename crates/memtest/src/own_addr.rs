use core::ptr::{read_volatile, write_volatile};
use core::{intrinsics::unlikely, range::RangeInclusive};
use arch::timer::Timer;
use uefi::boot::MemoryAttribute;
use uefi::mem::memory_map::{MemoryMap, MemoryMapOwned};
use uefi_graphics2::UefiDisplay;
use embedded_graphics::{geometry::Point, prelude::RgbColor};
use embedded_graphics::mono_font::ascii::{FONT_6X10, FONT_8X13};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;
use alloc::format;
use arch::mem::{check_addr, read, write};

pub fn own_addr_test(range: RangeInclusive<usize>, region: MemoryMapOwned, display: &mut UefiDisplay) { 
    own_addr_fill(range, &region, display);
    own_addr_check(range, region, display);
}



pub fn own_addr_fill(range: RangeInclusive<usize>, region: &MemoryMapOwned, display: &mut UefiDisplay) {
    for i in range.into_iter() {
        let mem = match region.get(i) {
            Some(v) => v,
            None => {
                continue;
            },
        };

        if !check_addr(mem.virt_start as usize) {
            continue;
        }
        let addr = mem.virt_start as *mut u64; 
        
        unsafe {
            write_volatile(addr, mem.virt_start);
        }
    }
 }

pub fn own_addr_check(range: RangeInclusive<usize>, region: MemoryMapOwned, display: &mut UefiDisplay) {
    for i in range.into_iter() {
        let mem = match region.get(i) {
            Some(v) => v,
            None => {
                continue;
            },
        };
        if !check_addr(mem.virt_start as usize) {
            continue;
        }
        let addr = mem.virt_start as *const u64;
        unsafe {
            let val = read_volatile(addr);

            if val != mem.virt_start {
                // Do
            }
        }
    }   
}
