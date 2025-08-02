use crate::mem::paging::{get_kernel_map, map};
use arch::{msr::wrmsr, rdmsr, tsc::TSC_TICKS_PER_MS};
use boot::HIGHER_HALF_OFFSET;
use core::{fmt::Write, time::Duration};
use fb::println;
use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{Page, PageTableFlags, PhysFrame, Size4KiB},
};

const APIC_BASE_MSR: u32 = 0x1B;
const TSC_DEADLINE_MSR: u32 = 0x6E0;

pub struct LocalApic {
    address: u64,
    #[allow(dead_code)]
    frequency: u64,
}

impl LocalApic {
    pub fn new() -> Self {
        let mut apic_base = rdmsr(APIC_BASE_MSR);

        apic_base &= !(1 << 8);
        apic_base &= !(1 << 11);

        println!("Apic base: 0x{:x}", apic_base);

        map::<Size4KiB>(
            Page::from_start_address(VirtAddr::new(apic_base + *HIGHER_HALF_OFFSET)).unwrap(),
            PhysFrame::from_start_address(PhysAddr::new(apic_base)).unwrap(),
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_EXECUTE,
            true,
        );

        let lapic = LocalApic {
            address: apic_base + *HIGHER_HALF_OFFSET,
            frequency: 0,
        };

        lapic.enable_apic();

        lapic
    }
    #[allow(dead_code)]
    pub fn read(&self, reg: u64) -> u32 {
        unsafe { ((self.address + reg) as *const u32).read_volatile() }
    }
    pub fn write(&self, reg: u64, val: u32) {
        unsafe {
            ((self.address + reg) as *mut u32).write_volatile(val);
        }
    }
    #[allow(dead_code)]
    pub fn siv(&self) -> u32 {
        self.read(LapicReg::SPURIOUS)
    }
    pub fn set_siv(&self, val: u32) {
        self.write(LapicReg::SPURIOUS, val);
    }
    pub fn enable_apic(&self) {
        self.set_siv(0x1ff);

        self.write(LapicReg::TPR, 0);
    }
    pub fn deadline(&self, int_no: u8, deadline: &Duration) {
        self.write(LapicReg::TIMER, (2 << 17) | int_no as u32);

        wrmsr(
            TSC_DEADLINE_MSR,
            deadline.as_millis() as u64 * *TSC_TICKS_PER_MS,
        );
    }
}

pub struct LapicReg;

impl LapicReg {
    pub const SPURIOUS: u64 = 0xF0;
    pub const TPR: u64 = 0x80;
    pub const TIMER: u64 = 0x320;
}
