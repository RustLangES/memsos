use sync::Once;
use limine::memory_map::Entry;
use alloc::vec::Vec;


const ALIGNMENT: usize = core::mem::align_of::<u64>();

pub type MemoryMap = &'static [&'static Entry];

pub static MEMORY_MAP: Once<MemoryMap> = Once::new();

#[derive(Debug, Clone, Copy)]
pub struct MemoryEntry {
    pub start: u64,
    pub end: u64,
}

pub trait MemModule {
    const NAME: &'static str;
    
    fn init(memory_map: MemoryMap) -> Self;
    fn run(&mut self, reports: &mut Vec<MemoryReport>);
}

#[derive(Debug)]
pub enum MemoryError {
    StuckAt
}

#[derive(Debug)]
pub struct MemoryReport {
    pub address: u64,
    pub kind: MemoryError
}

pub fn init_mem_module(memory: MemoryMap) {
    MEMORY_MAP.call_once(|| memory);
}

pub fn load_memtest<T: MemModule>(reports: &mut Vec<MemoryReport>) {
   let mut test = T::init(*MEMORY_MAP);
   test.run(reports);
}

pub fn check_addr(ptr: *mut u64) -> bool {
    ptr as usize % ALIGNMENT == 0 && !ptr.is_null()
}
