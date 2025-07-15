use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use lazy_static::lazy_static;
use core::fmt::Write;
use crate::println;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.stack_segment_fault.set_handler_fn(stack_segment_fault);
        idt.page_fault.set_handler_fn(page_fault);

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn page_fault(stack_frame: InterruptStackFrame, code: PageFaultErrorCode) {
    panic!("Page fault\n{:?}\n{:?}", stack_frame, code);
}

extern "x86-interrupt" fn stack_segment_fault(stack_frame: InterruptStackFrame, code: u64) {
    panic!("Stack segment fault\nframe:{:?}\ncode:{:?}",stack_frame, code);
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("Interrupt: {:?}", stack_frame);
}
