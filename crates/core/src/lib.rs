#![no_std]

mod test;

use crate::test::marchc;
use crate::test::pattern;
use core::fmt::Arguments;
use core::ops::{Add, AddAssign};

pub struct TestResult {
    pub bad_addrs: u64,
}

impl TestResult {
    pub fn new() -> Self {
        Self { bad_addrs: 0 }
    }
}

impl Add for TestResult {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        TestResult {
            bad_addrs: self.bad_addrs + rhs.bad_addrs,
        }
    }
}

impl AddAssign for TestResult {
    fn add_assign(&mut self, rhs: Self) {
        self.bad_addrs += rhs.bad_addrs;
    }
}

pub fn run_test<M: Mem, L: Logger>(logger: &mut L, mem: &M, region: &MemoryRegion) -> TestResult {
    logger.log(format_args!("Checking region {:?}", region));
    let mut result = TestResult { bad_addrs: 0 };

    logger.ui_change_test("March-C");

    result += marchc::run_march_c(mem, region);

    logger.ui_change_test("Pattern test, own address");

    result += pattern::run_test_own_address(mem, region);

    logger.ui_change_test("Pattern test, rand number");

    result += pattern::run_test_rand_num(mem, region);

    result
}

#[derive(Debug)]
pub struct MemoryRegion {
    pub start: u64,
    pub end: u64,
}

pub trait Mem {
    fn parse(&self, region: &MemoryRegion) -> MemoryRegion;
    fn check(&self, addr: u64) -> bool;
    fn read(&self, addr: u64) -> u64;
    fn write(&self, addr: u64, value: u64);
}

pub trait Logger {
    fn log(&self, message: Arguments<'_>);
    fn ui_change_test(&mut self, test: &str);
}
