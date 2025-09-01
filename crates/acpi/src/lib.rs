#![no_std]
#![feature(str_from_raw_parts)]

pub mod table;

use arch::paging::map;
use boot::{HIGHER_HALF_OFFSET, requests::RSDP_REQUEST};
use core::fmt::Write;
use fb::println;
use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{Page, PageTableFlags, PhysFrame, Size4KiB},
};

use crate::table::{AcpiTables, RsdpHeader, SdtHeader};

pub fn init_acpi() {
    let a = RSDP_REQUEST.get_response().unwrap().address() as u64;

    let b = (a.wrapping_add(*HIGHER_HALF_OFFSET) & !0xfff) as *mut RsdpHeader;

    let a_aligned = a & !0xfff;

    map::<Size4KiB>(
        Page::from_start_address(VirtAddr::new(b as u64)).unwrap(),
        PhysFrame::from_start_address(PhysAddr::new(a_aligned)).unwrap(),
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
        true,
    )
    .unwrap();

    let acpi = unsafe { AcpiTables::new(b) };

    let ptr = &acpi.rsdt as *const SdtHeader;
    let ptr = (ptr as u64) as *mut [u8; 70];

    println!("{:?}", unsafe { ptr.read_unaligned() });

    for entry in acpi.get_tables() {
        let a = (entry as u64 + *HIGHER_HALF_OFFSET) as *mut [u8; 4];
        let b = unsafe { (a).read_volatile() };
        println!("{:?}", b);
    }

    println!("{:?}", acpi.rsdt);
}
