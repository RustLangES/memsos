use core::arch::asm;
use heapless::Vec;

const VEC_SIZE: usize = 121;
const CPUID_FLAG_MSR: u32 = 1 << 5;

#[derive(Debug)]
pub struct CpuInfo {
    pub vendor: Vendor,
    pub model: &'static str,
    pub family: u32,
    pub features: Vec<Feature, 121>,
    pub stepping: u32,
}

impl CpuInfo {
    pub fn has_feature(&self, feat: Feature) -> bool {
        self.features.contains(&feat)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Feature {
    Msr,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Vendor {
    Intel,
    Amd,
    #[cfg(target_arch = "aarch64")]
    Arm,
    #[cfg(target_arch = "riscv64")]
    RiscV,
    Unknown,
}

impl CpuInfo {
    pub fn new() -> Self {
        let vendor = get_vendor();
        let family = get_cpu_family();
        let stepping = get_cpu_stepping();
        let model = get_cpu_model();
        let features = get_features();

        Self {
            vendor,
            family,
            stepping,
            model,
            features,
        }
    }
}

impl Default for CpuInfo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_arch = "x86_64")]
#[derive(Debug)]
struct CpuId {
    edx: u32,
    ecx: u32,
    ebx: u32,
    eax: u32,
}

#[allow(unused_assignments)]
#[cfg(target_arch = "x86_64")]
fn cpuid(mode: u32) -> CpuId {
    let mut edx: u32 = 0;
    let mut ecx: u32 = 0;
    let mut ebx: u32 = 0;
    let mut eax: u32 = 0;

    unsafe {
        asm!(
            "mov eax, {0:e}",
            "cpuid",
            "mov {1:e}, edx",
            "mov {2:e}, ecx",
            "mov {3:e}, ebx",
            "mov {4:e}, eax",
            in(reg) mode,
            out(reg) edx,
            out(reg) ecx,
            out(reg) ebx,
            out(reg) eax,
        );
    }

    CpuId { edx, ecx, ebx, eax }
}

#[cfg(target_arch = "x86_64")]
#[inline(never)]
pub fn get_vendor() -> Vendor {
    let result = cpuid(0);

    let vendor = [result.ebx, result.edx, result.ecx];

    let bytes1 = vendor[0].to_ne_bytes();
    let bytes2 = vendor[1].to_ne_bytes();
    let bytes3 = vendor[2].to_ne_bytes();

    let mut combined = [0u8; 12];
    combined[..4].copy_from_slice(&bytes1);
    combined[4..8].copy_from_slice(&bytes2);
    combined[8..12].copy_from_slice(&bytes3);

    let s = core::str::from_utf8(&combined).unwrap();

    match s {
        "GenuineIntel" => Vendor::Intel,
        "AuthenticAMD" => Vendor::Amd,
        "AMDisbetter!" => Vendor::Amd,
        _ => Vendor::Unknown,
    }
}

#[cfg(target_arch = "aarch64")]
pub fn get_vendor() -> Vendor {
    Vendor::Arm
}

#[cfg(target_arch = "riscv64")]
pub fn get_vendor() -> Vendor {
    Vendor::RiscV
}

#[cfg(target_arch = "x86_64")]
pub fn get_cpu_family() -> u32 {
    let result = cpuid(1);

    let family = (result.eax >> 8) & 0x0f;
    let extended_family = (result.eax >> 20) & 0xff;

    if family == 15 {
        family + extended_family
    } else {
        family
    }
}

pub fn get_features() -> Vec<Feature, VEC_SIZE> {
    let result = cpuid(1);
    let mut features = Vec::new();

    if result.edx & CPUID_FLAG_MSR == 32 {
        features.push(Feature::Msr).unwrap();
    }

    features
}

#[cfg(target_arch = "aarch64")]
pub fn get_cpu_family() -> u32 {
    let midr: u64;
    unsafe {
        asm!("mrs {0}, MIDR_EL1", out(reg) midr);
    }
    ((midr >> 4) & 0xFF) as u32
}

#[cfg(target_arch = "riscv64")]
pub fn get_cpu_family() -> u32 {
    let marchid: u64;
    unsafe {
        asm!("csrr {}, marchid", out(reg) marchid);
    }
    marchid as u32
}

#[cfg(target_arch = "x86_64")]
pub fn get_cpu_stepping() -> u32 {
    let result = cpuid(1);
    result.eax & 0xf
}

#[cfg(target_arch = "aarch64")]
pub fn get_cpu_stepping() -> u32 {
    let midr: u64;
    unsafe {
        asm!("mrs {0}, MIDR_EL1", out(reg) midr);
    }
    (midr & 0xF) as u32
}

#[cfg(target_arch = "riscv64")]
pub fn get_cpu_stepping() -> u32 {
    let mimpid: u64;
    unsafe {
        asm!("csrr {}, mimpid", out(reg) mimpid);
    }
    mimpid as u32
}

#[cfg(target_arch = "x86_64")]
#[inline(never)]
pub fn get_cpu_model() -> &'static str {
    let result = cpuid(1);

    let base_model = (result.eax >> 4) & 0xF;

    let base_family = (result.eax >> 8) & 0xF;
    let extended_model = (result.eax >> 16) & 0xF;
    let extended_family = (result.eax >> 20) & 0xFF;

    let model = if base_family == 0x06 || base_family == 0x0F {
        (extended_model << 4) | base_model
    } else {
        base_model
    };
    let family = if base_family == 0x0F {
        base_family + extended_family
    } else {
        base_family
    };

    let vendor = get_vendor();

    // TODO: add more models
    match vendor {
        Vendor::Intel => match (family, model) {
            (6, _) => "Intel Core i3",
            (0x0F, _) => "Intel Core i7",
            _ => "???",
        },
        Vendor::Amd => match (family, model) {
            (23, _) => "Amd ryzen",
            _ => "???",
        },
        _ => "Unknown Vendor",
    }
}

#[cfg(target_arch = "aarch64")]
pub fn get_cpu_model() -> &'static str {
    let midr: u64;
    unsafe {
        asm!("mrs {0}, MIDR_EL1", out(reg) midr);
    }
    let implementer = (midr >> 24) & 0xFF;
    let part_number = (midr >> 4) & 0xFFF;

    match (implementer, part_number) {
        (0x41, 0xD03) => "Cortex-A53",
        (0x41, 0xD07) => "Cortex-A57",
        (0x41, 0xD08) => "Cortex-A72",
        (0x41, 0xD0C) => "Cortex-A76",
        _ => "Unknown ARM CPU",
    }
}

#[cfg(target_arch = "riscv64")]
pub fn get_cpu_model() -> &'static str {
    let marchid: u64;
    unsafe {
        asm!("csrr {}, marchid", out(reg) marchid);
    }

    match marchid {
        0x80000000 => "SiFive U74",
        0x80000001 => "SiFive U54",
        0x80000002 => "SiFive S51",
        _ => "Unknown RISC-V CPU",
    }
}
