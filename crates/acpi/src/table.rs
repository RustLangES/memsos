use arch::paging::map;
use boot::HIGHER_HALF_OFFSET;
use core::fmt::Write;
use core::str;
use fb::println;

use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{Page, PageTableFlags, PhysFrame, Size4KiB},
};

#[derive(Debug)]
pub enum AcpiVersion {
    V1,
    V2,
}

#[repr(C, packed(1))]
pub struct AcpiTables {
    pub rsdp: RsdpHeader,
    pub rsdt: SdtHeader,
}

impl AcpiTables {
    // NOTE: THIS FUNCTION EXPECTS RSDP_ADDRESS IS MAPPED
    pub unsafe fn new(rsdp_address: *mut RsdpHeader) -> Self {
        let rsdp = unsafe { rsdp_address.read_unaligned() };
        let rsdt = unsafe { rsdp.get_rsdt_address().read_unaligned() };

        Self { rsdp, rsdt }
    }
    pub fn get_tables(&self) -> impl Iterator<Item = usize> {
        let entry_size = if self.rsdp.revision == 0 { 4 } else { 8 };
        let mut table_entries_ptr = unsafe {
            ((self.rsdp.rsdt_adddress as *mut u32).byte_add(size_of::<SdtHeader>())).cast::<u8>()
        };
        let mut num_entries = (self.rsdt.len as usize - size_of::<SdtHeader>()) / entry_size;

        core::iter::from_fn(move || {
            if num_entries > 0 {
                unsafe {
                    let entry = if entry_size == 4 {
                        table_entries_ptr.cast::<u32>() as usize
                    } else {
                        table_entries_ptr.cast::<u64>() as usize
                    };
                    table_entries_ptr = table_entries_ptr.byte_add(entry_size);
                    num_entries -= 1;

                    let b = (entry as u64).wrapping_add(*HIGHER_HALF_OFFSET) & !0xfff;

                    let a_aligned = entry as u64 & !0xfff;

                    match map::<Size4KiB>(
                        Page::from_start_address(VirtAddr::new(b as u64)).unwrap(),
                        PhysFrame::from_start_address(PhysAddr::new(a_aligned)).unwrap(),
                        PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
                        false,
                    ) {
                        Ok(_) => {
                            return Some(b as usize);
                        }
                        Err(_) => {
                            return Some(entry);
                        }
                    };
                }
            } else {
                None
            }
        })
    }
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

#[derive(Debug, Clone, Copy)]
#[repr(C, packed(1))]
pub struct SdtHeader {
    pub signature: [u8; 4],
    pub len: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub oem_table_id: [u8; 8],
    pub oem_revision: u32,
    pub creator_id: [u8; 4],
    pub creator_revision: u32,
}

#[derive(Debug)]
#[repr(C, packed(1))]
pub struct FadtHeader {
    pub sdt: SdtHeader,
    pub firmware_ctrl: u32,
    pub dsdt: u32,

    reserved: u8,

    pub prefered_power_management_profile: u8,
    pub sci_interrupt: u16,
    pub smi_interrupt: u32,
    pub acpi_enable: u8,
    pub acpi_disable: u8,
    pub s4bios_req: u8,

    pub pstate_control: u8,
    pub pm1a_event_block: u32,
    pub pm1b_event_block: u32,
    pub pm1a_control_block: u32,
    pub pm1b_control_block: u32,
    pub pm2_control_block: u32,
    pub pmtimer_block: u32,

    pub gpe0_block: u32,
    pub gpe1_block: u32,
    pub pm1_event_length: u8,
    pub pm2_control_length: u8,
    pub pm_timer_length: u8,
    pub gpe0_length: u8,
    pub gpe1_length: u8,
    pub gpe1_base: u8,
    pub cstate_control: u8,
    pub worst_c2_latency: u16,
    pub worst_c3_latency: u16,
    pub flush_size: u16,
    pub flush_stride: u16,
    pub duty_offset: u8,
    pub duty_width: u8,
    pub day_alarm: u8,
    pub month_alarm: u8,
    pub century: u8,

    // ACPI 2.0
    pub boot_architecture_flags: u16,

    pub reserved2: u8,
    pub flags: u32,

    pub reset_reg: GenericAddress,

    pub x_firmware_control: u64,
    pub s_dsdt: u64,

    pub x_pm1a_event_block: GenericAddress,
    pub x_pm1b_event_block: GenericAddress,
    pub x_pm1a_control_block: GenericAddress,
    pub x_pm1b_control_block: GenericAddress,
    pub x_pm2_control_block: GenericAddress,
    pub x_pmtimer_block: GenericAddress,
    pub x_gpe_0_block: GenericAddress,
    pub x_gpe_1_block: GenericAddress,
}

impl FadtHeader {
    pub fn check_signature(&self) {
        unsafe {
            if str::from_raw_parts(self.sdt.signature.as_ptr(), self.sdt.signature.len()) != "FACP"
            {
                panic!("Invalid FADT")
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed(1))]
pub struct GenericAddress {
    pub address_space: u8,
    pub bit_width: u8,
    pub bit_offset: u8,
    pub access_size: u8,
    pub address: u64,
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
        )
        .unwrap();

        b
    }
    pub fn get_xsdt_address() -> *mut () {
        todo!();
    }
}
