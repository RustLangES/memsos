#![no_std]
#![feature(str_from_raw_parts)]

pub mod hpet;
pub mod table;

use boot::{HIGHER_HALF_OFFSET, requests::RSDP_REQUEST};
use core::str;
use paging::map;
use sync::Once;
use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{Page, PageTableFlags, PhysFrame, Size4KiB},
};

use crate::{
    hpet::{HpetHeader, init_hpet},
    table::{AcpiTables, FadtHeader, RsdpHeader, SdtHeader},
};

pub static ACPI_TABLE: Once<AcpiTables> = Once::new();

macro_rules! thaterror {
    (pub enum $s_name:ident { $( #[error($msg:expr)] $name: ident ),* }) => {
        #[derive(Debug)]
        pub enum $s_name {
            $(
                $name,
            )*
        }

        impl ::core::fmt::Display for $s_name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    $(
                        $s_name::$name => f.write_str($msg),
                    )*
                }
            }
        }

        impl ::core::error::Error for $s_name {}
    }
}

thaterror! {
    pub enum AcpiError {
        #[error("No acpi tables")]
        NoAcpiTables,

        #[error("Invalid Checksum")]
        InvalidChecksum,

        #[error("Invalid SDT")]
        InvalidSdt,

        #[error("Invalid Rsdp")]
        InvalidRsdp
    }
}

pub fn init_acpi() -> Result<(u8, [u8; 6]), AcpiError> {
    let a = RSDP_REQUEST
        .get_response()
        .ok_or(AcpiError::NoAcpiTables)?
        .address() as u64;

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

    let mut acpi = unsafe { AcpiTables::new(b)? };
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

                init_hpet(b).unwrap();

                hpet = Some(b);
            }
            _ => {
                continue;
            }
        }
    }

    acpi.hpet = hpet;

    ACPI_TABLE.call_once(|| acpi);

    Ok((power_profile, ACPI_TABLE.rsdp.oem_id))
}
