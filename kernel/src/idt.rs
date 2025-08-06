use arch::{
    hcf::hcf,
    tsc::{TSC_TICKS_PER_MS, rdtsc},
};
use core::fmt::Write;
use fb::println;
use lazy_static::lazy_static;

use x86_64::{
    registers::control::Cr2,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode},
};

use crate::x2apic::X2APIC;

pub const TIMER_VECTOR: u8 = 40;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode);

        idt[TIMER_VECTOR].set_handler_fn(x2apic_handle);

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn x2apic_handle(_stack_frame: InterruptStackFrame) {
    X2APIC.eoi();
}

extern "x86-interrupt" fn invalid_opcode(stack_frame: InterruptStackFrame) {
    panic!("Invalid opcode!\n{:?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    //println!("PAGE FAULT!!");
    //println!("Accesed_address: {:?}", Cr2::read());
    //println!("Error Code: {:?}", error_code);
    //println!("{:#?}", stack_frame);
    hcf();
}
