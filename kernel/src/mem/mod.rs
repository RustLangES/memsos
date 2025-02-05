use core::sync::atomic::{AtomicU64, Ordering};
use memsos_core::Mem;

#[derive(Debug)]
pub struct MemWriter {
    offset: AtomicU64,
}

impl Mem for MemWriter {
    fn read(&self, addr: u64) -> u64 {
        let ptr = addr as *const u64;

        unsafe {
            ptr.read()
        }
    }
    fn write(&self, addr: u64, value: u64) {
        let ptr = addr as *mut u64;

        unsafe {
            *ptr = value;
        }
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
