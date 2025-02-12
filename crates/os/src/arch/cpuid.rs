use heapless::String;
use core::arch::asm;

#[derive(Debug)]
pub struct CpuInfo {
    pub vendor: Vendor,
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

        Self {
            vendor
        }
    }
}

impl CpuInfo {}
    
fn cpuid(eax: u32) -> CpuId {
    let mut edx: u32 = 0;
    let mut ecx: u32 = 0;
    let mut ebx: u32 = 0;

    unsafe {
        asm!(
            "mov eax, {0:e}",
            "cpuid",
            "mov {1:e}, edx",
            "mov {2:e}, ecx",
            "mov {3:e}, ebx",
            in(reg) eax,
            out(reg) edx,
            out(reg) ecx,
            out(reg) ebx, 
        );
    }

    CpuId {
        edx,
        ecx,
        ebx
    }
}

pub fn get_vendor() -> Vendor {
        let result = cpuid(0);
        let vendor = [result.ebx, result.edx, result.ecx];

        let bytes: &[u8; 12] = unsafe {
            core::mem::transmute(&vendor)
        };

        let s = unsafe { core::str::from_utf8_unchecked(bytes) };
        match s {
            "GenuineIntel" => Vendor::Intel,
            "AuthenticAMD" => Vendor::Amd, // new amd vendor
            "AMDisbetter!" => Vendor::Amd, // old amd vendor
            _ => Vendor::Unknown(s)
        }
    }


struct CpuId {
    edx: u32,
    ecx: u32,
    ebx: u32,
}
