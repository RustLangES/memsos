use core::{ffi::c_void, sync::atomic::{AtomicUsize, Ordering}};

const SIZE_IN_BYTES: usize = 71680;
const SIZE_IN_PAGES: usize = 71680 / 4096;

use r_efi::efi::{self, SystemTable};

pub struct BumpAllocator {
    pub cursor: AtomicUsize,
    pub block: *mut u8,
}

impl BumpAllocator {
    pub const fn new() -> BumpAllocator {
        Self {
            cursor: AtomicUsize::new(0),
            block: core::ptr::null_mut(),
        }
    }
    pub fn init(&mut self, st: *mut SystemTable) {
        unsafe {
            let block = &mut self.block;
            let ptr = block as *mut *mut u8;
            let status = ((*(*st).boot_services).allocate_pool)(efi::LOADER_DATA, SIZE_IN_PAGES, ptr.cast::<*mut c_void>());
            
            assert!(!status.is_error(), "Cannot init memory allocator");

            let addr = self.block.addr();
            self.cursor = AtomicUsize::new(addr);
            assert_ne!(self.block, core::ptr::null_mut());
        }
    }
    pub fn free(&self, st: *mut SystemTable) {
        unsafe {
            ((*(*st).boot_services).free_pages)(self.block as u64, SIZE_IN_PAGES);
        }
    }
}

unsafe impl core::alloc::GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let cursor = self.cursor.load(Ordering::SeqCst);
        let align_mask = !(layout.align() - 1);
        let next_ptr = cursor.checked_sub(layout.size()).expect("Cannot allocate more memory") & align_mask;
        let end = (self.block as usize) + SIZE_IN_BYTES;

        
        assert!(!(next_ptr > end), "No more memory");

        self.cursor.store(next_ptr, Ordering::SeqCst);
        next_ptr as *mut u8
        
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        // auch
    }
}
