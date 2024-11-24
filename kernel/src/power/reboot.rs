use core::arch::asm;

use crate::asm::{inb::inb, outb::outb};

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
