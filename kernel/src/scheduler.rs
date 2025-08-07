use core::arch::asm;
use core::fmt::Write;
use fb::println;
use x86_64::VirtAddr;

#[derive(Debug)]
#[repr(C)]
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
        }
    }
}

pub struct Process {
    pub start: VirtAddr,
    pub context: Context,
}

#[unsafe(naked)]
pub extern "C" fn fill_context(ctx: *mut Context) {
    core::arch::naked_asm!(
        "mov [rdi + 0x00], rax",
        "mov [rdi + 0x08], rbx",
        "mov [rdi + 0x10], rcx",
        "mov [rdi + 0x18], rdx",
        "mov [rdi + 0x20], rsi",
        "mov [rdi + 0x30], rbp",
        "mov [rdi + 0x38], rsp",
        "mov [rdi + 0x40], r8",
        "mov [rdi + 0x48], r9",
        "mov [rdi + 0x50], r10",
        "mov [rdi + 0x58], r11",
        "mov [rdi + 0x60], r12",
        "mov [rdi + 0x68], r13",
        "mov [rdi + 0x70], r14",
        "mov [rdi + 0x78], r15",
        "ret",
    );
}

impl Process {
    pub fn new(start: VirtAddr) -> Self {
        let mut ctx = Context::default();
        let rdi: u64;

        unsafe {
            asm!("mov {}, rdi", out(reg) rdi);
        }

        fill_context((&mut ctx) as *mut Context);
        ctx.rdi = rdi;
        ctx.rip = start.as_u64();

        println!("{:?}", ctx);
        Self {
            start,
            context: ctx,
        }
    }
    pub fn run(&self) {
        unsafe {
            asm!("jmp {}", in(reg) self.context.rip);
        }
    }
    pub fn stop(&mut self, rip: VirtAddr) {
        //self.context.rip = rip;
    }
}
