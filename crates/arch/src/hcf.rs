use core::arch::asm;

pub fn hcf() -> ! {
    unsafe {
        asm!("hlt");
        core::hint::unreachable_unchecked()
    }
}
