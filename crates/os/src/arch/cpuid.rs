use core::arch::asm;

#[derive(Debug)]
pub struct CpuInfo {
    pub vendor: Vendor,
    pub model: &'static str,
    pub family: u32,
    pub stepping: u32,
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
        let stepping = get_cpu_stepping();
        let model = get_cpu_model();

        Self {
            vendor,
            family,
            stepping,
            model,
        }
    }
}

impl Default for CpuInfo {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(unused_assignments)]
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

pub fn get_cpu_stepping() -> u32 {
    let result = cpuid(1);
    result.eax & 0xf
}

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

struct CpuId {
    edx: u32,
    ecx: u32,
    ebx: u32,
    eax: u32,
}
