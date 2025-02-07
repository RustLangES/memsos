use crate::{Mem, MemoryRegion, TestResult};
use rand::{SeedableRng, RngCore};
use rand_chacha::ChaCha20Rng;

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

pub fn run_test_rand_num<M: Mem>(mem: &M, region: &MemoryRegion) -> TestResult {
    let offset_region = mem.parse(&region);
    let mut rand = ChaCha20Rng::seed_from_u64(12381293);
    let mut bad_addrs = 0;

    for addr in offset_region.start..offset_region.end {
        let pattern = &rand.next_u64();
        if !mem.check(addr) {
            continue;
        }     

        mem.write(addr, *pattern);
        
        if mem.read(addr) != *pattern {
            bad_addrs += 1;
        }
    } 

    TestResult {
        bad_addrs
    }
}


