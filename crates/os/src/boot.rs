use crate::request::{BOOT_INFO_REQUEST, HHDM_REQUEST, MEMORY_MAP_REQUEST};
use limine::{memory_map::Entry, response::BootloaderInfoResponse};

pub struct BootInfo<'a> {
    pub info: &'a BootloaderInfoResponse,
    pub memory_regions: &'a [&'a Entry],
    pub offset: u64,
}

impl<'a> BootInfo<'a> {
    pub fn get() -> Self {
        let info = BOOT_INFO_REQUEST.get_response().unwrap();
        let memory_regions = MEMORY_MAP_REQUEST.get_response().unwrap().entries();
        let offset = HHDM_REQUEST.get_response().unwrap().offset();

        Self {
            info,
            memory_regions,
            offset,
        }
    }
}
