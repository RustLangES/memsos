pub mod read;
pub mod write;
use bootloader_api::info::{MemoryRegion, MemoryRegionKind};
use core::sync::atomic::{AtomicU64, Ordering};
use heapless::Vec;

pub static MEMORY_WRITER: Mem = Mem::create();

pub struct Mem {
    offset: AtomicU64,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum MemError {
    NullPtr,
    MisalignedPtr,
}

impl Mem {
    pub const fn create() -> Self {
        Self {
            offset: AtomicU64::new(0),
        }
    }
    pub fn init(&self, offset: u64) {
        self.offset.store(offset, Ordering::SeqCst);
    }
    pub fn write(&self, addr: u64, value: u64) -> Result<(), MemError> {
        let offset = self.offset.load(Ordering::SeqCst);

        let ptr = (addr + offset) as *mut u64;

        if ptr as usize % core::mem::align_of::<u64>() != 0 {
            return Err(MemError::MisalignedPtr);
        }

        unsafe {
            ptr.write(value);
        }

        Ok(())
    }
    pub fn read(&self, addr: u64) -> Result<u64, MemError> {
        let offset = self.offset.load(Ordering::SeqCst);
        let ptr = (addr + offset) as *const u64;

        if ptr.is_null() {
            return Err(MemError::NullPtr);
        }

        if ptr as usize % core::mem::align_of::<u64>() != 0 {
            return Err(MemError::MisalignedPtr);
        }

        let value = unsafe { ptr.read() };

        Ok(value)
    }
}

pub fn init_mem(offset: u64) {
    MEMORY_WRITER.init(offset);
}
