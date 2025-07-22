use core::arch::asm;

#[inline]
pub fn wrmsr(msr: u32, value: u64) {
    let lo = value as u32;
    let hi = (value >> 32) as u32;

    unsafe {
        asm!("wrmsr", in("ecx") msr, in("eax") lo, in("edx") hi);
    }
}

#[inline]
pub fn rdmsr(msr: u32) -> u64 {
    let (hi, lo): (u32, u32);

    unsafe {
        asm!("rdmsr", out("eax") lo, out("edx") hi, in("ecx") msr);
    }

    ((hi as u64) << 32) | (lo as u64)
}
