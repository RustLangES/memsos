use heapless::Vec;

const VEC_MAX_SIZE: usize = 1024;

pub struct MemResult {
    pub pattern: u8,
    pub memory_regions_tested: Vec<u16, VEC_MAX_SIZE>,
    pub passed: bool,
}

pub fn test_memory() -> MemResult {
    // in this moment we only make a very simple test in 0x1000-0x1100
    let pattern = 0xFFFF;
    let mut memory_regions_tested: Vec<u16, VEC_MAX_SIZE> = Vec::new();

    for mem in (0x1000..0x1100).step_by(0x1) {
       memory_regions_tested.push(mem); 
    }

    MemResult {
        pattern,
        memory_regions_tested,
        passed: false,
    }
}
