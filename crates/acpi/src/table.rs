use arch::paging::map;
use core::str;

use boot::HIGHER_HALF_OFFSET;
use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{Page, PageTableFlags, PhysFrame, Size4KiB},
};

#[derive(Debug)]
pub enum AcpiVersion {
    V1,
    V2,
}

#[derive(Debug)]
#[repr(C, packed(1))]
pub struct RsdpHeader {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_adddress: u32,

    pub len: u32,
    pub xsdt_address: u64,
    pub extended_checksum: u8,
    pub reserved: [u8; 3],
}

#[repr(C, packed(1))]
pub struct SdtHeader {
    pub signature: [u8; 4],
    pub len: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub oem_table_id: [u8; 8],
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_revision: u32,
}

impl SdtHeader {
    pub fn check_signature(&self) {
        unsafe {
            if str::from_raw_parts(self.signature.as_ptr(), self.signature.len()) != "RSDT" {
                panic!("Invalid RSDT")
            }
        }
    }
}

pub fn get_tables_v1(rsdt: &SdtHeader) -> &[u32] {
    let total_length = rsdt.len as usize;
    let header_size = core::mem::size_of::<SdtHeader>();
    let pointer_count = (total_length - header_size) / 4;

    unsafe {
        let ptr = (rsdt as *const SdtHeader as *const u8).add(header_size) as *const u32;
        core::slice::from_raw_parts(ptr, pointer_count)
    }
}

pub fn get_tables_v2() {
    todo!();
}

impl RsdpHeader {
    pub fn check_signature(&self) {
        unsafe {
            if str::from_raw_parts(self.signature.as_ptr(), self.signature.len()) != "RSD PTR " {
                panic!("Invalid RSDP")
            }
        }
    }
    pub fn get_version(&self) -> AcpiVersion {
        match self.revision {
            0 | 1 => AcpiVersion::V1,
            _ => AcpiVersion::V2,
        }
    }
    pub fn get_rsdt_address(&self) -> *mut SdtHeader {
        let b = ((self.rsdt_adddress as u64).wrapping_add(*HIGHER_HALF_OFFSET) & !0xfff)
            as *mut SdtHeader;

        let a_aligned = self.rsdt_adddress as u64 & !0xfff;

        map::<Size4KiB>(
            Page::from_start_address(VirtAddr::new(b as u64)).unwrap(),
            PhysFrame::from_start_address(PhysAddr::new(a_aligned)).unwrap(),
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
            true,
        );

        b
    }
    pub fn get_xsdt_address() -> *mut () {
        todo!();
    }
}
