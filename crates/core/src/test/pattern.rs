use crate::{Mem, MemoryRegion, TestResult};

pub fn run_test_own_address<M: Mem>(mem: &M, region: &MemoryRegion) -> TestResult {
    let offset_region = mem.parse(&region);
    let mut bad_addrs = 0;

    for addr in offset_region.start..offset_region.end {
        if !mem.check(addr) {
            continue;
        }     

        mem.write(addr, addr);
        
        if mem.read(addr) != addr {
            bad_addrs += 1;
        }
    } 

    TestResult {
        bad_addrs
    }
}

