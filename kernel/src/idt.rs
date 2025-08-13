use arch::{
    hcf::hcf,
    tsc::{TSC_TICKS_PER_MS, rdtsc},
};
use core::{fmt::Write, time::Duration};
use embedded_graphics::prelude::Point;

use fb::{get_fb_writer, get_ui_writer, println};
use lazy_static::lazy_static;
use ui::{components::placeholder::PlaceholderText, get_ui_state};

use x86_64::{
    registers::control::Cr2,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode},
};

use x2apic::X2APIC;

pub const TIMER_VECTOR: u8 = 40;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode);
        idt.double_fault.set_handler_fn(double_fault);

        idt[TIMER_VECTOR].set_handler_fn(x2apic_handle);

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn x2apic_handle(stack_frame: InterruptStackFrame) {
    X2APIC.eoi();
    let state = get_ui_state();
    state.test_info_section.update_time();
    if state.test_info_section.time.enabled {
        X2APIC.oneshot(TIMER_VECTOR, Duration::from_secs(1));
    }
}

extern "x86-interrupt" fn double_fault(stack_frame: InterruptStackFrame, code: u64) -> ! {
    panic!("Double fault!\n{:?}\ncode: {code}", stack_frame);
}

extern "x86-interrupt" fn invalid_opcode(stack_frame: InterruptStackFrame) {
    panic!("Invalid opcode!\n{:?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    println!("PAGE FAULT!!");
    println!("Accesed_address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    hcf();
}
