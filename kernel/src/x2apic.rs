// Based from https://docs.rs/x86/0.52.0/src/x86/apic/x2apic.rs.html
use arch::{msr::wrmsr, rdmsr};
use bit_field::BitField;
use sync::Once;

pub const IA32_APIC_BASE: u32 = 0x1b;
pub const IA32_X2APIC_SIVR: u32 = 0x80f;
pub const IA32_X2APIC_LVT_LINT0: u32 = 0x835;
pub const IA32_X2APIC_LVT_TIMER: u32 = 0x832;
pub const IA32_TSC_DEADLINE: u32 = 0x6e0;
pub const IA32_X2APIC_EOI: u32 = 0x80b;

pub static X2APIC: Once<X2Apic> = Once::new();

pub fn init_x2apic() {
    X2APIC.call_once(X2Apic::new);
}

pub struct X2Apic {
    pub base: u64,
}

impl X2Apic {
    pub fn new() -> Self {
        let mut base = rdmsr(IA32_APIC_BASE);
        base.set_bit(10, true);
        base.set_bit(11, true);

        wrmsr(IA32_APIC_BASE, base);

        let svr: u64 = 1 << 8 | 15;

        wrmsr(IA32_X2APIC_SIVR, svr);

        let lint0 = 1 << 16 | (1 << 15) | (0b111 << 8) | 0x20;

        wrmsr(IA32_X2APIC_LVT_LINT0, lint0);

        Self { base }
    }
    pub fn tsc_enable(&self, vector: u8) {
        wrmsr(IA32_TSC_DEADLINE, 0);

        let mut lvt: u64 = rdmsr(IA32_X2APIC_LVT_TIMER);
        lvt &= !0xff;
        lvt |= vector as u64;

        lvt.set_bit(16, false);

        lvt.set_bit(17, false);

        lvt.set_bit(18, true);

        wrmsr(IA32_X2APIC_LVT_TIMER, lvt);
    }
    pub fn tsc_set(&self, value: u64) {
        mfence();
        wrmsr(IA32_TSC_DEADLINE, value);
    }
    pub fn eoi(&self) {
        wrmsr(IA32_X2APIC_EOI, 0);
    }
}

fn mfence() {
    unsafe { core::arch::asm!("mfence") };
}
