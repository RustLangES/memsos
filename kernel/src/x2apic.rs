use arch::{msr::wrmsr, rdmsr};
use bit_field::BitField;

pub const IA32_APIC_BASE: u32 = 0x1b;
pub const IA32_X2APIC_SIVR: u32 = 0x80f;
pub const IA32_X2APIC_LVT_LINT0: u32 = 0x835;

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
}
