use core::arch::asm;

pub fn sti() {
    unsafe {
        asm!("sti");
    }
}

pub fn cli() {
    unsafe {
        asm!("cli");
    }
}
