use sync::Once;
use limine::memory_map::Entry;

pub type MemoryMap = &'static [&'static Entry];

pub static MEMORY_MAP: Once<MemoryMap> = Once::new();

pub fn init_mem_module(memory: MemoryMap) {
    MEMORY_MAP.call_once(|| memory);
}

pub fn load_memtest<T: MemModule>() {
   let mut test = T::init(*MEMORY_MAP);
   test.run();
}

#[derive(Debug, Clone, Copy)]
pub struct MemoryEntry {
    pub start: u64,
    pub end: u64,
}

pub trait MemModule {
    const NAME: &'static str;
    
    fn init(memory_map: MemoryMap) -> Self;
    fn run(&mut self);
}
