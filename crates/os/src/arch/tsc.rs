use core::arch::asm;
pub fn rdtsc() -> u64 {
    let low: u32;
    let high: u32;
    unsafe {
        asm!(
            "rdtsc",
            out("eax") low,
            out("edx") high,
            options(nomem, nostack),
        );
    }
    (high as u64) << 32 | (low as u64)
}
