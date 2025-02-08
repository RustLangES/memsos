use memsos_core::Mem;

const ALIGNMENT: usize = core::mem::align_of::<u64>();

#[derive(Debug)]
pub struct MemWriter {
    offset: u64,
}

impl Mem for MemWriter {
    fn read(&self, addr: u64) -> u64 {
        let ptr = addr as *mut u64;

        unsafe { ptr.read() }
    }
    fn write(&self, addr: u64, value: u64) {
        let ptr = addr as *mut u64;

        unsafe {
            ptr.write_volatile(value);
        }
    }
    fn check(&self, addr: u64) -> bool {
        let ptr = addr as *mut u64;
        addr as usize % ALIGNMENT == 0 && !ptr.is_null()
    }
    fn parse(&self, region: &memsos_core::MemoryRegion) -> memsos_core::MemoryRegion {
        memsos_core::MemoryRegion {
            start: region.start + self.offset,
            end: region.end + self.offset,
        }
    }
}

impl MemWriter {
    pub const fn create(offset: u64) -> Self {
        Self { offset }
    }
}
