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

    let header = unsafe { (*entry).table_address } as *const SmbiosHeader;

    crate::render!(&crate::text!((0, 0), "{:?}", unsafe { *header }));

    Ok(entry)
}
