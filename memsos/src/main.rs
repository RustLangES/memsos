#![feature(uefi_std)]

extern crate alloc;
use alloc::format;

use arch::mem::{get_usable_mem, write};
use arch::protocols::{self, UefiProtocols};
use embedded_graphics::prelude::RgbColor;
use memtest::own_addr::own_addr_check;
use uefi::mem::memory_map::MemoryMap as MemoryMapTrait;
use std::os::uefi as uefi_std;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicI32, AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::Duration;
use uefi::boot::{
    create_event, get_handle_for_protocol, memory_map, open_protocol_exclusive, set_timer, EventNotifyFn, EventType, MemoryType, Tpl
};
use uefi::proto::console::gop::GraphicsOutput;
use uefi::runtime::ResetType;
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

static COUNT: AtomicI32 = AtomicI32::new(0);

fn main() {
    setup();

    boot::set_watchdog_timer(0, 0x10000, None).unwrap();

    let mut gop = protocols::UefiProtocols::get().gop;
    let mode = gop.current_mode_info();
    let mut display = UefiDisplay::new(gop.frame_buffer(), mode).unwrap();
    
    let style = MonoTextStyle::new(&FONT_6X10, Rgb888::WHITE);
    let mut text = Text::new("Hello World!", Point { x: 30, y: 100 }, style);

    text.draw(&mut display).unwrap();
    display.flush();
     
    uefi::runtime::reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
}

