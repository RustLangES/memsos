use core::ptr::NonNull;

use acpi_parser::{AcpiHandler, AcpiTables, PhysicalMapping};
use boot::{HIGHER_HALF_OFFSET, requests::RSDP_REQUEST};
use core::fmt::Write;
use fb::println;
use sync::Once;
use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{Page, PageTableFlags, PhysFrame, Size4KiB},
};

use crate::mem::paging::get_kernel_map;

pub static ACPI: Once<Acpi> = Once::new();

pub fn init_acpi() {
    ACPI.call_once(|| Acpi::new());
}

#[derive(Clone)]
struct AcpiMapper;

impl AcpiHandler for AcpiMapper {
    unsafe fn map_physical_region<T>(
        &self,
        physical_address: usize,
        size: usize,
    ) -> PhysicalMapping<Self, T> {
        unsafe {
            PhysicalMapping::new(
                physical_address,
                NonNull::new_unchecked((physical_address + *HIGHER_HALF_OFFSET as usize) as *mut _),
                size,
                size,
                self.clone(),
            )
        }
    }
    fn unmap_physical_region<T>(_region: &PhysicalMapping<Self, T>) {}
}

#[repr(packed)]
pub struct Rsdp {
    signature: [u8; 8],
    checksum: u8,
    oemid: [char; 6],
    revision: u8,
    rsdt_address: u32, // deprecated, in acpi 2.0
}

pub struct Acpi {
    tables: AcpiTables<AcpiMapper>,
}

impl Acpi {
    pub fn new() -> Self {
        let address = RSDP_REQUEST.get_response().unwrap().address();
        //let mut kernel_mapper = get_kernel_map();
        println!("{:X}", address);
        let rsdp_ptr = (address) as *const Rsdp;
        let rsdp = unsafe { rsdp_ptr.read() };
        if rsdp.revision == 0 {
            println!("Acpi 1.0");
        } else if rsdp.revision == 2 {
            println!("Acpi 2.0!");
        }

        loop {}
    }
}
