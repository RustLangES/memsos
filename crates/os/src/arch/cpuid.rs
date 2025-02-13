use core::arch::asm;
use heapless::String;

#[derive(Debug)]
pub struct CpuInfo {
    pub vendor: Vendor,
    pub family: u32,
}

#[derive(Debug)]
pub enum Vendor {
    Intel,
    Amd,
    Unknown(&'static str),
}

impl CpuInfo {
    pub fn new() -> Self {
        let vendor = get_vendor();
        let family = get_cpu_family();

        Self { vendor, family }
    }
}

impl CpuInfo {}

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

pub fn get_vendor() -> Vendor {
    let result = cpuid(0);
    let vendor = [result.ebx, result.edx, result.ecx];

    let bytes: &[u8; 12] = unsafe { core::mem::transmute(&vendor) };

    let s = unsafe { core::str::from_utf8_unchecked(bytes) };
    match s {
        "GenuineIntel" => Vendor::Intel,
        "AuthenticAMD" => Vendor::Amd, // new amd vendor
        "AMDisbetter!" => Vendor::Amd, // old amd vendor
        _ => Vendor::Unknown(s),
    }
}

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


struct CpuId {
    edx: u32,
    ecx: u32,
    ebx: u32,
    eax: u32
}

