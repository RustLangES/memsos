use core::arch::asm;

pub fn hcf() -> ! {
    unsafe {
        loop {
            asm!("hlt");
        }
    }
}
