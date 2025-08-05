use core::arch::asm;

/// # Panics
///
/// It panics if value can't be u32
#[inline]
pub fn wrmsr(msr: u32, value: u64) {
    let lo = value as u32;
    let hi = (value >> 32) as u32;

    unsafe {
        asm!("wrmsr", in("ecx") msr, in("eax") lo, in("edx") hi);
    }
}

#[must_use]
#[inline]
pub fn rdmsr(msr: u32) -> u64 {
    let (hi, lo): (u32, u32);

    unsafe {
        asm!("rdmsr", out("eax") lo, out("edx") hi, in("ecx") msr);
    }

    (u64::from(hi) << 32) | u64::from(lo)
}
