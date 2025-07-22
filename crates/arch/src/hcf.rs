use core::arch::asm;

pub fn hcf() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
