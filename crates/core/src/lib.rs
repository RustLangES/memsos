#![no_std]

mod test;
use crate::test::marchc;
use crate::test::pattern;
use core::fmt::Arguments;
use core::fmt::Error;
use core::ops::{Add, AddAssign};
use heapless::String;

#[derive(Default)]
pub struct TestResult {
    pub bad_addrs: u64,
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

pub fn run_test<M: Mem, L: Logger>(
    logger: &mut L,
    mem: &M,
    region: &MemoryRegion,
    kind: MemTestKind,
) -> TestResult {
    logger.log(format_args!("Checking region {:?}", region));
    let mut result = TestResult::default();

    if kind == MemTestKind::Basic || kind == MemTestKind::Advanced {
        logger.ui_change_test("March-C");

        result += marchc::run_march_c(mem, region);

        logger.ui_change_test("Pattern test, own address");

        result += pattern::run_test_own_address(mem, region);
    }

    if kind == MemTestKind::Advanced {
        logger.ui_change_test("Pattern test, rand number");

        result += pattern::run_test_rand_num(mem, region);
    }

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MemTestKind {
    Basic,
    Advanced,
}

impl TryFrom<String<256>> for MemTestKind {
    type Error = Error;
    fn try_from(value: String<256>) -> Result<Self, Self::Error> {
        let s = value;
        Ok(match s.as_str() {
            "basic" => Self::Basic,
            "advanced" => Self::Advanced,
            _ => {
                return Err(Error);
            }
        })
    }
}
