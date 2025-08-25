#![no_std]

use arch::paging::map;
use boot::{HIGHER_HALF_OFFSET, requests::RSDP_REQUEST};
use core::fmt::Write;
use fb::println;
use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{Page, PageTableFlags, PhysFrame, Size4KiB},
};

#[derive(Debug)]
#[repr(C)]
pub struct RsdpHeader {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oemid: [u8; 6],
    pub revision: u8,
}

pub fn init_acpi() {
    let a = RSDP_REQUEST.get_response().unwrap().address() as u64;
    let b = (a.wrapping_add(*HIGHER_HALF_OFFSET) & !0xfff) as *mut RsdpHeader;

    let a_aligned = a & !0xfff;

    map::<Size4KiB>(
        Page::from_start_address(VirtAddr::new(b as u64)).unwrap(),
        PhysFrame::from_start_address(PhysAddr::new(a_aligned)).unwrap(),
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
        false,
    );

    println!("{:?}", unsafe { b.read_unaligned() });

    println!("{:x}", a);
}
