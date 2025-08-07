use x86_64::VirtAddr;

pub struct Context {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,

    pub rip: u64,
    pub rflags: u64,
    pub cr3: u64,

    pub cs: u64,
    pub ss: u64,
}

impl Context {
    pub fn load(&self) {
        let rax: u64;
        let rbx: u64;
        let rcx: u64;
        let rdx: u64;
        let rsi: u64;
        let rdi: u64;
        let rbp: u64;
        let rsp: u64;
        let r8: u64;
        let r9: u64;
        let r10: u64;
        let r11: u64;
        let r12: u64;
        let r13: u64;
        let r14: u64;
        let r15: u64;
        let rip: u64;
        let rflags: u64;
        let cr3: u64;
        let cs: u64;
        let ss: u64;
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            rax: 0,
            rbx: 0,
            rcx: 0,
            rdx: 0,
            rsi: 0,
            rdi: 0,
            rbp: 0,
            rsp: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,

            rip: 0,
            rflags: 0,
            cr3: 0,

            cs: 0,
            ss: 0,
        }
    }
}

pub struct Process {
    pub start: VirtAddr,
    pub context: Context,
}

impl Process {
    pub fn new(start: VirtAddr) -> Self {
        Self {
            start,
            context: Context::default(),
        }
    }
    pub fn run(&self) {
        self.context.load();
        unsafe {
            core::arch::asm!("jmp {0}", in(reg) self.start.as_u64(), options(noreturn));
        }
    }
    pub fn stop() {
        todo!();
    }
}
