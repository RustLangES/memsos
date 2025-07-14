#![no_std]
#![no_main]
#![feature(sync_unsafe_cell)]

mod boot;
mod writer;

use core::panic::PanicInfo;

use crate::writer::{get_ui, init_writer};

#[unsafe(no_mangle)]
pub extern "C" fn kmain() {
    init_writer();
    get_ui().write_str("Hello from memsos again;");

    loop {}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
