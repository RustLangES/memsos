use crate::asm::{inb::inb, outb::outb};

#[derive(Debug)]
pub struct Port(pub u16);

impl Port {
    pub fn read(&self) -> u8 {
        inb(self.0)
    }
    pub fn write(&self, value: u8) {
        outb(self.0, value);
    }
}
