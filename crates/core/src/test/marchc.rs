use crate::{Mem, MemoryRegion};

pub fn run_march_c<M: Mem>(mem: &M, region: &MemoryRegion) {
    let offset_region = mem.parse(region);

    for addr in offset_region.start..offset_region.end {
        if !mem.check(addr) {
            continue;
        }
        mem.write(addr, 0);
    }

    for addr in offset_region.start..offset_region.end {
        if !mem.check(addr) {
            continue;
        }
        if mem.read(addr) != 0 {
            panic!("Test failed at address 1 {:?}", addr);
        }
        mem.write(addr, 1);
    }

    for addr in (offset_region.start..offset_region.end).rev() {
        if !mem.check(addr) {
            continue;
        }
        if mem.read(addr) != 1 {
            panic!("Test failed at address 2 {:?}", addr);
        }
        mem.write(addr, 0);
    }

    for addr in offset_region.start..offset_region.end {
        if !mem.check(addr) {
            continue;
        }
        if mem.read(addr) != 0 {
            panic!("Test failed at address 3 {:?}", addr);
        }
        mem.write(addr, 1);
    }
}
