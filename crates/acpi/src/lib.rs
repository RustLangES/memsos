#![no_std]
#![feature(str_from_raw_parts)]

pub mod hpet;
pub mod table;

use arch::paging::map;
use boot::{HIGHER_HALF_OFFSET, requests::RSDP_REQUEST};
use core::str;
use sync::Once;
use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{Page, PageTableFlags, PhysFrame, Size4KiB},
};

use crate::{
    hpet::HpetHeader,
    table::{AcpiTables, FadtHeader, RsdpHeader, SdtHeader},
};

pub static ACPI_TABLE: Once<AcpiTables> = Once::new();

pub fn init_acpi() -> (u8, [u8; 6]) {
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

    let mut power_profile: u8 = 0;

    let mut acpi = unsafe { AcpiTables::new(b) };
    let mut hpet = None;

    for entry in acpi.get_tables() {
        let a = entry as *mut SdtHeader;

        let b = unsafe { a.read_volatile() };

        match unsafe { str::from_raw_parts(b.signature.as_ptr(), b.signature.len()) } {
            "FACP" => {
                let a = entry as *mut FadtHeader;

                let b = unsafe { a.read_volatile() };

                power_profile = b.prefered_power_management_profile;
            }
            "HPET" => {
                let a = entry as *mut HpetHeader;

                let b = unsafe { a.read_volatile() };

                hpet = Some(b);
            }
            _ => {
                continue;
            }
        }
    }

    acpi.hpet = hpet;

    ACPI_TABLE.call_once(|| acpi);

    (power_profile, ACPI_TABLE.rsdp.oem_id)
}
