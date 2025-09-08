use core::fmt::Write;
use core::str;
use fb::println;

use crate::table::{GenericAddress, SdtHeader};

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
