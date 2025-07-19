#![no_std]

use commons::mem::{MemModule, MemoryMap};

pub struct MarchC {
    mem_map: MemoryMap
}

impl MemModule for MarchC {
    const NAME: &'static str = "March C";
    
    fn init(memory_map: MemoryMap) -> Self {
        Self { mem_map: memory_map } 
    }

    fn run(&mut self) {
        todo!(); 
    }
}
