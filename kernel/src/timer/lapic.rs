use arch::msr::rdmsr;
use boot::HIGHER_HALF_OFFSET;
use x86_64::{structures::paging::{Page, PageTableFlags, PhysFrame, Size4KiB}, PhysAddr, VirtAddr};
use crate::mem::paging::get_kernel_map;
use fb::println;
use core::fmt::Write;

const APIC_BASE_MSR: u32 = 0x1B;

pub struct LocalApic {
    address: u64,
    frequency: u64,
}

impl LocalApic {
    pub fn new() -> Self {
        let mut apic_base = rdmsr(APIC_BASE_MSR);
        let mut mapper = get_kernel_map();

        apic_base &= !(1 << 8);
        apic_base &= !(1 << 11);

        println!("Apic base: 0x{:x}", apic_base);

        mapper.map::<Size4KiB>(
            PhysFrame::containing_address(PhysAddr::new(apic_base)),
            Page::containing_address(VirtAddr::new(apic_base + *HIGHER_HALF_OFFSET)),
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_EXECUTE,
            true
        );

        let mut lapic = LocalApic {
            address: apic_base + *HIGHER_HALF_OFFSET,
            frequency: 0,
        };

        lapic.enable_apic();
        
        lapic
    }
    pub fn read(&self, reg: u64) -> u32 {
        unsafe { 
            ((self.address + reg) as *const u32).read_volatile()
        }
    }
    pub fn write(&self, reg: u64, val: u32) {
        unsafe {
            ((self.address + reg) as *mut u32).write_volatile(val);
        }
    }
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
}

pub struct LapicReg;

impl LapicReg {
    pub const SPURIOUS: u64 = 0xF0;
    pub const TPR: u64 = 0x80;
}
