use bit_field::BitField;
use boot::HIGHER_HALF_OFFSET;
use core::fmt::Write;
use core::str;
use fb::println;
use paging::map;
use sync::Once;
use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{Page, PageTableFlags, PhysFrame, Size4KiB},
};

use crate::table::{GenericAddress, SdtHeader};

pub const HPET_CONFIGURATION_REGISTER: u64 = 0x10;
pub const HPET_MAIN_COUNTER_REGISTER: u64 = 0xF0;

pub static HPET_STATE: Once<HpetState> = Once::new();
static HPET_ADDR: Once<u64> = Once::new();

pub struct HpetState {
    pub frequency: u64,
    pub period: u64,
    pub clock_tick_unit: u16,
}
// inspired by: https://docs.rs/acpi/latest/src/acpi/sdt/hpet.rs.html#37-67

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct HpetHeader {
    pub sdt: SdtHeader,
    pub event_timer_block_id: u32,
    pub base_address: GenericAddress,
    pub hpet_number: u8,
    pub clock_tick_unit: u16,
    /// Bits `0..4` specify the page protection guarantee. Bits `4..8` are reserved for OEM attributes.
    pub page_protection_and_oem: u8,
}

impl HpetHeader {
    pub fn check_signature(&self) {
        unsafe {
            if str::from_raw_parts(self.sdt.signature.as_ptr(), self.sdt.signature.len()) != "HPET"
            {
                println!("Invalid HPET");
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HpetInfo {
    pub hardware_rev: u8,
    pub num_comparators: u8,
    pub main_counter_is_64bits: bool,
    pub legacy_irq_capable: bool,
    pub pci_vendor_id: u16,
    pub base_address: u64,
    pub hpet_number: u8,
    pub clock_tick_unit: u16,
    pub page_protection: PageProtection,
}

#[derive(Debug, Clone, Copy)]
pub enum PageProtection {
    None,
    Protected4K,
    Protected64K,
    Other,
}

#[derive(Debug)]
pub enum HpetInfoError {
    InvalidHpetTable,
}

impl TryFrom<HpetHeader> for HpetInfo {
    type Error = HpetInfoError;

    fn try_from(header: HpetHeader) -> Result<Self, Self::Error> {
        if header.base_address.address_space != 0 {
            // todo;  add warn macro
            println!("HPET reported as not in system memory");
            return Err(HpetInfoError::InvalidHpetTable);
        }

        let event_timer_block_id = header.event_timer_block_id;

        Ok(Self {
            hardware_rev: event_timer_block_id.get_bits(0..8) as u8,
            num_comparators: event_timer_block_id.get_bits(8..13) as u8,
            main_counter_is_64bits: event_timer_block_id.get_bit(13),
            legacy_irq_capable: event_timer_block_id.get_bit(15),
            pci_vendor_id: event_timer_block_id.get_bits(16..32) as u16,
            base_address: header.base_address.address,
            hpet_number: header.hpet_number,
            clock_tick_unit: header.clock_tick_unit,
            page_protection: match header.page_protection_and_oem.get_bits(0..4) {
                0 => PageProtection::None,
                1 => PageProtection::Protected4K,
                2 => PageProtection::Protected64K,
                3..=15 => PageProtection::Other,
                _ => unreachable!(),
            },
        })
    }
}

pub fn init_hpet(hpet_header: HpetHeader) -> Result<(), HpetInfoError> {
    let info = HpetInfo::try_from(hpet_header)?;

    HPET_ADDR.call_once(|| info.base_address + *HIGHER_HALF_OFFSET);

    map::<Size4KiB>(
        Page::from_start_address(VirtAddr::new(info.base_address + *HIGHER_HALF_OFFSET)).unwrap(),
        PhysFrame::from_start_address(PhysAddr::new(info.base_address)).unwrap(),
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_EXECUTE,
        true,
    )
    .unwrap();

    let period = (read_hpet(0) >> 32) & u64::MAX;

    assert!(period != 0);
    assert!(period <= 0x05F5E100);

    let f = (u64::pow(10, 15) as f64 / period as f64) as u64;

    println!("Hpet Period: {}", period);
    println!("Hpet Frequency {}", f);

    write_hpet(
        HPET_CONFIGURATION_REGISTER,
        read_hpet(HPET_CONFIGURATION_REGISTER) | 1,
    );

    HPET_STATE.call_once(|| HpetState {
        frequency: f,
        period,
        clock_tick_unit: info.clock_tick_unit,
    });

    Ok(())
}

pub fn write_hpet(offset: u64, value: u64) {
    let ptr = *HPET_ADDR as *mut u64;

    unsafe {
        ptr.byte_add(offset as usize).write_volatile(value);
    }
}

pub fn read_hpet(offset: u64) -> u64 {
    let ptr = *HPET_ADDR as *mut u64;

    unsafe { ptr.byte_add(offset as usize).read_volatile() }
}
