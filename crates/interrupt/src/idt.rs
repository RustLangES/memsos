use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt.overflow.set_handler_fn(overflow_handler);
        idt.stack_segment_fault
            .set_handler_fn(stack_segment_fault_handler);

        idt
    };
}

extern "x86-interrupt" fn stack_segment_fault_handler(stack_frame: InterruptStackFrame, code: u64) {
    panic!("Stack Segment Fault {:#?}\n Code:{}", stack_frame, code);
}

extern "x86-interrupt" fn overflow_handler(stack_frame: InterruptStackFrame) {
    panic!("Overflow {:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    panic!("Double fault {}\n{:#?}", error_code, stack_frame)
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    panic!("Page fault:\n{:?}\nCode:\n{:?}", stack_frame, error_code);
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    panic!("Error: \n{:#?}", stack_frame);
}
