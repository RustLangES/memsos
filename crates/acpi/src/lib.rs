#![no_std]

use boot::{HIGHER_HALF_OFFSET, requests::RSDP_REQUEST};
use core::fmt::Write;
use fb::println;

#[derive(Debug)]
#[repr(C)]
pub struct RsdpHeader {
    pub signature: [char; 4],
}

pub fn init_acpi() {
    let a = RSDP_REQUEST.get_response().unwrap().address() as u64;
    let b = (a.wrapping_add(*HIGHER_HALF_OFFSET)) as *mut RsdpHeader;

    println!("{:?}", unsafe { b.read_volatile() });

    println!("{:x}", a);
}
