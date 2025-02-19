#[derive(Debug)]
pub struct Port(pub u16);

impl Port {
    pub fn read(&self) -> u8 {
        arch_specific::read_port(self.0)
    }
    pub fn write(&self, value: u8) {
        arch_specific::write_port(self.0, value);
    }
}

#[cfg(target_arch = "x86_64")]
mod arch_specific {
    use crate::asm::{inb::inb, outb::outb};

    pub fn read_port(port: u16) -> u8 {
        inb(port)
    }

    pub fn write_port(port: u16, value: u8) {
        outb(port, value);
    }
}

#[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
mod arch_specific {
    #[cfg(target_arch = "aarch64")]
    const UART0: *mut u8 = 0x0900_0000 as *mut u8; // PL011 UART (QEMU)

    #[cfg(target_arch = "riscv64")]
    const UART0: *mut u8 = 0x1001_3000 as *mut u8; // SiFive UART

    pub fn read_port(_port: u16) -> u8 {
        unsafe {
            while UART0.add(0x18).read_volatile() & 0x01 == 0 {}
            UART0.read_volatile()
        }
    }

    pub fn write_port(_port: u16, value: u8) {
        unsafe {
            while UART0.add(0x14).read_volatile() & 0x20 != 0 {}
            UART0.write_volatile(value);
        }
    }
}
