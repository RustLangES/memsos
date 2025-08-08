use x86_64::VirtAddr;
use x86_64::structures::idt::InterruptStackFrame;

use crate::process::Process;
use core::cell::SyncUnsafeCell;
use core::fmt::Write;
use core::time::Duration;
use fb::println;

pub const PROCESS_DEADLINE: Duration = Duration::from_millis(1_500);
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
}

impl Scheduler {
    pub const fn new(processes: [Process; 2]) -> Self {
        Self {
            processes,
            current_process: 0,
            first_run: true,
            running_critical_process: false,
        }
    }
    pub fn save(&mut self) {
        self.processes[self.current_process].fill_context();
    }
    pub fn call_next(&mut self, r: VirtAddr, stack_frame: InterruptStackFrame) {
        if self.first_run {
            self.first_run = false;
            return;
        }

        //self.processes[self.current_process].stop(r);

        self.current_process = (self.current_process + 1) % self.processes.len();

        let rip = self.processes[self.current_process].run();

        self.processes[self.current_process].context.rsp = stack_frame.stack_pointer.as_u64();

        println!("{}", stack_frame.instruction_pointer.as_u64());
        self.processes[self.current_process].context.rip = stack_frame.instruction_pointer.as_u64();

        let frame = InterruptStackFrame::new(
            VirtAddr::new(rip),
            stack_frame.code_segment,
            stack_frame.cpu_flags,
            VirtAddr::new(self.processes[self.current_process].context.rsp),
            stack_frame.stack_segment,
        );

        unsafe {
            self.processes[self.current_process].write_context();
            frame.iretq();
        }
    }
}
