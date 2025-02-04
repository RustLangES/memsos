use memsos_core::{Mem, MemError};
use core::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug)]
pub struct MemWriter {
    offset: AtomicU64,
}

impl Mem for MemWriter {
    fn read(&self, addr: u64) -> Result<u64, MemError> {
    let offset = self.offset.load(Ordering::SeqCst);
    let ptr_addr = addr + offset;
    
    if ptr_addr as usize % core::mem::align_of::<u64>() != 0 {
        return Err(MemError::MisalignedPtr(ptr_addr));
    }

    let ptr = ptr_addr as *const u64;

    if ptr.is_null() {
        return Err(MemError::NullPtr);
    }

    let value = unsafe { ptr.read() };

    Ok(value)
}

    fn write(&self, addr: u64, value: u64) -> Result<(), MemError> {
        let offset = self.offset.load(Ordering::SeqCst);
            let ptr_addr = addr + offset;

         if ptr_addr as usize % core::mem::align_of::<u64>() != 0 {
            return Err(MemError::MisalignedPtr(ptr_addr));
            }

        let ptr = (addr + offset) as *mut u64; 

        unsafe {
            ptr.write(value);
        }

        Ok(())

    }
}

impl MemWriter {
    pub const fn create(offset: u64) -> Self {
        Self {
            offset: AtomicU64::new(offset),
        }
    }
}

