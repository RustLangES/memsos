#![feature(abi_x86_interrupt)]
#![no_std]

#[cfg(target_arch = "x86_64")]
pub mod idt;

#[cfg(target_arch = "x86_64")]
pub fn handle_interrupts() {
    idt::IDT.load();
}

#[cfg(target_arch = "riscv64")]
pub fn handle_interrupts() {
    todo!();
}

#[cfg(target_arch = "aarch64")]
pub fn handle_interrupts() {
    todo!();
}
