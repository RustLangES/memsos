use super::cpuid::{CpuInfo, Feature};
use core::arch::asm;

pub fn has_msr() {
    let cpu_info = CpuInfo::new();

    if !cpu_info.has_feature(Feature::Msr) {
        panic!("Msr not available");
    }
}

#[inline]
pub fn wrmsr(msr: u32, value: u64) {
    let low = value as u32;
    let high = (value >> 32) as u32;
    unsafe {
        asm!("wrmsr", in("ecx") msr, in("eax") low, in("edx") high);
    }
}

#[inline]
pub fn rdmsr(msr: u32) -> (u32, u32) {
    let (high, low): (u32, u32);
    unsafe {
        asm!("rdmsr", out("eax") low, out("edx") high, in("ecx") msr);
    }
    (high, low)
}
