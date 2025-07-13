use core::ffi::c_void;

use r_efi::efi::{self, MemoryDescriptor, Status, SystemTable};

pub fn efi_mem_map_run(st: *mut SystemTable, memory_size: *mut usize, memory_map: *mut MemoryDescriptor, memory_map_key: *mut usize, memory_descriptor_size: *mut usize, memory_descriptor_version: *mut u32) -> Status {
    unsafe { 
        ((*(*st).boot_services).get_memory_map)(memory_size, memory_map, memory_map_key, memory_descriptor_size, memory_descriptor_version)
    }
}

pub fn get_mem_map_size(st: *mut SystemTable, memory_size: &mut usize, memory_map_key: &mut usize, memory_descriptor_size: &mut usize, memory_descriptor_version: &mut u32) -> Status {
    efi_mem_map_run(st, memory_size as *mut usize, core::ptr::null_mut(), memory_map_key as *mut usize, memory_descriptor_size as *mut usize, memory_descriptor_version as *mut u32)
}

pub fn get_mem_map(st: *mut SystemTable) -> *mut MemoryDescriptor {
    let mut memory_size: usize = 0;
    let mut memory_map_key: usize = 0;
    let mut memory_descriptor_size: usize = 0;
    let mut memory_descriptor_version: u32 = 0;
    let status = get_mem_map_size(st, &mut memory_size, &mut memory_map_key, &mut memory_descriptor_size, &mut memory_descriptor_version);
    assert!(status == efi::Status::BUFFER_TOO_SMALL);
    
    let mut memory_map: *mut MemoryDescriptor = core::ptr::null_mut();
    let ptr: *mut *mut c_void = &mut memory_map as *mut *mut MemoryDescriptor as *mut *mut c_void;

    unsafe {
        ((*(*st).boot_services).allocate_pool)(efi::LOADER_DATA, memory_size + 2 * memory_descriptor_size, ptr);
        assert_ne!(memory_map, core::ptr::null_mut())
    }

    efi_mem_map_run(st, &mut memory_size as *mut usize, memory_map, &mut memory_map_key as *mut usize, &mut memory_descriptor_size as *mut usize, &mut memory_descriptor_version as *mut u32);

    memory_map

}
