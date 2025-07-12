use r_efi::efi::{self, Status, SystemTable};

pub unsafe fn get_mem_map_size(st: *mut SystemTable, memory_size: *mut usize, memory_map_key: *mut usize, memory_descriptor_size: *mut usize, memory_descriptor_version: *mut u32) -> Status {
    ((*(*st).boot_services).get_memory_map)(memory_size, core::ptr::null_mut(), memory_map_key, memory_descriptor_size, memory_descriptor_version)
}
