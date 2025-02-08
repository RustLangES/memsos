use crate::{Mem, MemoryRegion, TestResult};

pub fn run_march_c<M: Mem>(mem: &M, region: &MemoryRegion) -> TestResult {
    let offset_region = mem.parse(region);
    let mut bad_addrs = 0;

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
            bad_addrs += 1;
        }
        mem.write(addr, 1);
    }

    for addr in (offset_region.start..offset_region.end).rev() {
        if !mem.check(addr) {
            continue;
        }
        if mem.read(addr) != 1 {
            bad_addrs += 1;
        }
        mem.write(addr, 0);
    }

    for addr in offset_region.start..offset_region.end {
        if !mem.check(addr) {
            continue;
        }
        if mem.read(addr) != 0 {
            bad_addrs += 1;
        }
        mem.write(addr, 1);
    }

    TestResult { bad_addrs }
}
