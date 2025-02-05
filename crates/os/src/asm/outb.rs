use core::arch::asm;

pub fn outb(port: u16, value: u8) {
    unsafe {
        asm!(
            "out dx, al",
            inout("dx") port => _,
            in("al") value
        );
    }
}
