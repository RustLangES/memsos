use crate::request;
use core::result::Result;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SmbiosEntry {
    pub anchor: [u8; 5],
    pub checksum: u8,
    pub entry_length: u8,
    pub major_version: u8,
    pub minor_version: u8,
    pub docrev: u8,
    pub entry_revision: u8,
    pub reserved: u8,
    pub max_structure_size: u32,
    pub table_address: u64,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct SmbiosHeader {
    pub smbios_type: u8,
    pub length: u8,
    pub handle: u16,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct SmbiosCpuInfo {
    pub type_: u8,
    pub length: u8,
    pub handle: u16,
    pub socket_designation: u8,
    pub processor_type: u8,
    pub processor_family: u8,
    pub processor_manufacturer: u8,
    pub processor_id_low: u32,
    pub processor_id_high: u32,
    pub processor_version: u8,
    pub voltage: u8,
    pub external_clock: u16,
    pub max_speed: u16,
    pub current_speed: u16,
    pub status: u8,
    pub processor_upgrade: u8,
}

#[derive(Debug)]
pub enum SmbiosError {
    NoSmbiosEntry,
    InvalidSmbiosEntry,
}

pub fn check_smbios() -> bool {
    let response = request::SMBIOS_REQUEST.get_response().unwrap();

    if let Some(_) = response.entry_64() {
        return true;
    }

    return false;
}

fn smbios_header_len(hd: &SmbiosHeader) -> usize {
    let strtab_start = unsafe { (hd as *const _ as *const u8).add(hd.length as usize) };
    let mut i = 0;

    let max_search = 2048;

    unsafe {
        while i < max_search - 1 {
            if *strtab_start.add(i) == 0 && *strtab_start.add(i + 1) == 0 {
                break;
            }
            i += 1;
        }
    }

    hd.length as usize + i + 2
}

pub fn read_smbios() -> Result<*const SmbiosEntry, SmbiosError> {
    if !check_smbios() {
        return Err(SmbiosError::NoSmbiosEntry);
    }

    let response = request::SMBIOS_REQUEST.get_response().unwrap();
    let addr = response.entry_64().unwrap();
    let entry = addr.as_ptr() as *const SmbiosEntry;

    if entry.is_null() {
        return Err(SmbiosError::NoSmbiosEntry);
    }

    if unsafe { (*entry).anchor } != *b"_SM3_" {
        return Err(SmbiosError::InvalidSmbiosEntry);
    }

    let table_addr = unsafe { (*entry).table_address } as *const SmbiosHeader;
    let mut current = unsafe { table_addr.read_volatile() };
    let mut addr = unsafe { (*entry).table_address };

    loop {
        if current.smbios_type == 4 {
            let cpu_info = unsafe { *(addr as *const SmbiosCpuInfo) };

            crate::render!(&crate::text!((0, 0), "{:?}", cpu_info));
            break;
        }

        addr += smbios_header_len(&current) as u64;

        current = unsafe { (addr as *const SmbiosHeader).read_volatile() };
    }

    Ok(entry)
}

fn read_smbios_cpu() {}
