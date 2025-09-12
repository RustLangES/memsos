#![no_std]

pub mod io;

#[derive(Debug)]
pub struct PciDevice {
    pub device: u8,
    pub bus: u8,
    pub device_id: u16,
    pub vendor_id: u16,
    pub class: u16,
    pub header_type: u8,
    pub bars: [u32; 6],
    pub supported_fns: [bool; 8],
    pub interrupt_line: u8,
    pub interrupt_pin: u8,
}
