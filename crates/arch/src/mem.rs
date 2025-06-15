use uefi::{boot::{memory_map, MemoryType}, mem::memory_map::MemoryMapOwned};

pub struct MemoryMap {
    pub region: MemoryMapOwned,
    pub unusable_mem: MemoryMapOwned,
}

impl MemoryMap {
    pub fn new() -> Self {
        let region = memory_map(MemoryType::CONVENTIONAL).unwrap();
        let unusable_mem = memory_map(MemoryType::UNUSABLE).unwrap();

        Self {
            region,
            unusable_mem
        }
    }
}

pub fn read(addr: usize) -> usize {
    let ptr = addr as *const u64;

    unsafe { ptr.read() as usize }
}

pub fn write(addr: usize, value: usize) {
    let ptr = addr as *mut u64;

    unsafe { ptr.write(value as u64); }
}
