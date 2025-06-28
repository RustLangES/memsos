#![feature(uefi_std, new_range_api, ptr_as_uninit)]

extern crate alloc;
use alloc::format;

use arch::mem::{get_usable_mem, write};
use arch::protocols::{self, UefiProtocols};
use memtest::march_c::march_c_test;
use core::range::RangeInclusive;
use arch::timer::Timer;
use embedded_graphics::mono_font::iso_8859_1::FONT_6X13;
use embedded_graphics::prelude::RgbColor;
use uefi::mem::memory_map::{MemoryMap as MemoryMapTrait, MemoryMapMut};
use std::fmt::format;
use std::os::uefi as uefi_std;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicI32, AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::Duration;
use uefi::boot::{
    allocate_pages, allocate_pool, create_event, get_handle_for_protocol, memory_map, open_protocol_exclusive, set_timer, AllocateType, EventNotifyFn, EventType, MemoryAttribute, MemoryType, Tpl
};
use uefi::proto::console::gop::GraphicsOutput;
use uefi::runtime::{set_virtual_address_map, ResetType};
use uefi::{boot, Event, Handle, Status};
use uefi_graphics2::UefiDisplay;
use embedded_graphics::geometry::Point;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;


fn setup() {
    let st = uefi_std::env::system_table();
    let ih = uefi_std::env::image_handle();

    unsafe {
        uefi::table::set_system_table(st.as_ptr().cast());

        let ih = Handle::from_ptr(ih.as_ptr().cast()).unwrap();
        uefi::boot::set_image_handle(ih);
    }
}

fn main() {
    setup();

    boot::set_watchdog_timer(0, 0x10000, None).unwrap();

    let mut gop = protocols::UefiProtocols::get().gop;
    let mode = gop.current_mode_info();
    let mut display = UefiDisplay::new(gop.frame_buffer(), mode).unwrap();
    
    let style = MonoTextStyle::new(&FONT_6X13, Rgb888::WHITE);

    let map = get_usable_mem();

    let mut text = Text::new("No test running", Point { x: 0, y: 10 }, style);
    
    let timer = Timer::new();
     for region in map {
        if region.is_none() { continue; }

        let mut r = region.unwrap();
    
        text.text = "Own addr test";
        text.draw(&mut display).unwrap();
        display.flush();
        let buffer = unsafe { r.buffer_mut() };
        buffer.fill(0);
        march_c_test(buffer.as_mut_ptr(), buffer.len()-1);
    }
    let end = timer.elapsed();
 
    let mut text = Text::new("The end", Point { x: 30, y: 100 }, style);

    text.draw(&mut display).unwrap();
    display.flush();

    loop {}
    uefi::runtime::reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
}

