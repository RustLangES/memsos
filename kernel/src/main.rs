#![no_std]
#![no_main]
#![feature(sync_unsafe_cell)]

mod boot;
mod writer;

use core::fmt::Write;
use core::panic::PanicInfo;

use crate::writer::{get_ui, init_writer};

#[unsafe(no_mangle)]
pub extern "C" fn kmain() {
    init_writer();
    println!("Test"); 

    loop {}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
