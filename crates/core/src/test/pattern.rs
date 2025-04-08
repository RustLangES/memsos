use crate::{Mem, MemoryRegion, TestResult};
use core::sync::atomic::{AtomicU64, Ordering};

pub fn run_test_own_address<M: Mem>(mem: &M, region: &MemoryRegion) -> TestResult {
    let offset_region = mem.parse(region);
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

    TestResult { bad_addrs }
}

static RSEED: AtomicU64 = AtomicU64::new(0);
const RAND_MAX: u64 = (1_u64 << 31) - 1;

#[inline]
fn rand() -> u64 {
    let rseed = RSEED.load(Ordering::SeqCst);
    let rand = (rseed * 214013 + 12345) & RAND_MAX;

    RSEED.store(rand, Ordering::SeqCst);

    rand
}

pub fn run_test_rand_num<M: Mem>(mem: &M, region: &MemoryRegion) -> TestResult {
    let offset_region = mem.parse(region);
    let mut bad_addrs = 0;

    for addr in offset_region.start..offset_region.end {
        let pattern = rand();
        if !mem.check(addr) {
            continue;
        }

        mem.write(addr, pattern);

        if mem.read(addr) != pattern {
            bad_addrs += 1;
        }
    }

    TestResult { bad_addrs }
}
