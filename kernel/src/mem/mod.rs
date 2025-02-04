use core::sync::atomic::{AtomicU64, Ordering};
use memsos_core::{Mem, MemError};

#[derive(Debug)]
pub struct MemWriter {
    offset: AtomicU64,
}

impl Mem for MemWriter {
    fn read(&self, addr: u64) -> Result<u64, MemError> {
        let ptr = addr as *mut u64;

        if ptr as usize % core::mem::align_of::<u64>() != 0 {
            return Err(MemError::MisalignedPtr(ptr as u64));
        }

        if ptr.is_null() {
            return Err(MemError::NullPtr);
        }

        let value = unsafe { ptr.read() };

        Ok(value)
    }

    fn write(&self, addr: u64, value: u64) -> Result<(), MemError> {
        let ptr = addr as *mut u64;


        if ptr as usize % core::mem::align_of::<u64>() != 0 {
            return Err(MemError::MisalignedPtr(ptr as u64));
        }

       
        unsafe {
            ptr.write(value);
        }

        Ok(())
    }
    fn parse(&self, region: memsos_core::MemoryRegion) -> memsos_core::MemoryRegion {
        let offset = self.offset.load(Ordering::SeqCst);
        memsos_core::MemoryRegion {
            start: region.start + offset,
            end: region.end + offset
        }
    }
}

impl MemWriter {
    pub const fn create(offset: u64) -> Self {
        Self {
            offset: AtomicU64::new(offset),
        }
    }
}
