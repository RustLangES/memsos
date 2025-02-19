use core::arch::asm;

#[cfg(target_arch = "x86_64")]
use crate::asm::{inb::inb, outb::outb};

#[cfg(target_arch = "x86_64")]
pub fn reboot() {
    let mut good: u8 = 0x02;
    while good & 0x02 != 0 {
        good = inb(0x64);
    }
    outb(0x64, 0xFE);

    unsafe {
        asm!("hlt");
    }
}

#[cfg(target_arch = "aarch64")]
pub fn reboot() {
    unsafe {
        asm!("svc #0");
    }
}

#[cfg(target_arch = "riscv64")]
pub fn reboot() {
    unsafe {
        asm!("ebreak");
    }
}
