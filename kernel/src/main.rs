#![no_std]
#![no_main]
#![feature(sync_unsafe_cell, abi_x86_interrupt)]

mod boot;
mod tables;
mod writer;

use core::fmt::Write;
use core::panic::PanicInfo;

use crate::tables::idt::init_idt;
use crate::writer::init_writer;

#[unsafe(no_mangle)]
pub extern "C" fn kmain() {
    init_writer();
    init_idt();

    x86_64::instructions::interrupts::int3();

    println!("Test"); 

    loop {}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
