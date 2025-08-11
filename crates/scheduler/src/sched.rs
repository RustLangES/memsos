use x2apic::X2APIC;
use x86_64::VirtAddr;
use x86_64::structures::idt::InterruptStackFrame;

use crate::process::{Process, ProcessType};
use core::cell::SyncUnsafeCell;
use core::fmt::Write;
use core::time::Duration;
use fb::println;

pub const PROCESS_DEADLINE: Duration = Duration::from_millis(100);
pub static SCHEDULER: SyncUnsafeCell<Option<Scheduler>> = SyncUnsafeCell::new(None);

pub fn init_scheduler(processes: [Process; 2]) {
    unsafe {
        *SCHEDULER.get() = Some(Scheduler::new(processes));
    }
}

pub fn get_shed() -> &'static mut Scheduler {
    unsafe { SCHEDULER.get().as_mut().unwrap().as_mut().unwrap() }
}

pub struct Scheduler {
    pub processes: [Process; 2],
    pub current_process: usize,
    pub first_run: bool,
    pub running_critical_process: bool,
    pub enabled: bool,
}

impl Scheduler {
    pub const fn new(processes: [Process; 2]) -> Self {
        Self {
            processes,
            current_process: 0,
            first_run: true,
            running_critical_process: false,
            enabled: true,
        }
    }
    pub fn save(&mut self) {
        self.processes[self.current_process].fill_context();
    }
    pub fn get(&self) -> Process {
        self.processes[self.current_process]
    }
    pub fn call_next(&mut self, stack_frame: InterruptStackFrame) {
        if !self.enabled {
            return;
        }

        if !self.first_run {
            //   self.processes[self.current_process].context.rsp = stack_frame.stack_pointer.as_u64();
            self.processes[self.current_process].context.rip =
                stack_frame.instruction_pointer.as_u64();

            // self.processes[self.current_process].fill_context();

            self.current_process = (self.current_process + 1) % self.processes.len();
        }

        X2APIC.oneshot(40, PROCESS_DEADLINE);

        let frame = InterruptStackFrame::new(
            VirtAddr::new(process_jmp as u64),
            stack_frame.code_segment,
            stack_frame.cpu_flags,
            VirtAddr::new(self.processes[self.current_process].context.rsp),
            stack_frame.stack_segment,
        );

        unsafe {
            println!("iretq!");
            frame.iretq();
        }
    }
}

pub fn process_jmp() -> ! {
    get_shed().first_run = false;
    let process = get_shed().get();

    //process.write_context();
    println!("Jumpp!");
    //println!("{:x}", process.context.rsp);
    unsafe {
        core::arch::asm!("jmp {}", in(reg) process.context.rip, options(noreturn));
    }
}
