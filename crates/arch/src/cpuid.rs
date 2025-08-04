use core::arch::x86_64::CpuidResult;

macro_rules! make_vendor_enum {
    ($($arg:ident, $val: expr),*) => {
        #[derive(Debug)]
        pub enum CpuVendor {
            $(
                $arg,
            )*
            Unknown
        }

        impl<'a> From<&'a str> for CpuVendor {
            fn from(value: &'a str) -> Self {
                match value {
                    $(
                        $val => Self::$arg,
                    )*
                    _ => Self::Unknown,
                }
            }

        }
    };
}

macro_rules! make_feature_enum {
    ($($arg:ident, $val: expr),*) => {
        #[repr(u32)]
        pub enum CpuFeature {
            $(
                $arg = $val,
            )*
        }
    };
}

make_feature_enum! {
    X2Apic, 1 << 21,
    Tsc, 1 << 4,
    Msr, 1 << 5
}

make_vendor_enum! {
    Intel, "GenuineIntel",
    Amd, "AuthenticAMD",
    AmdOld, "AMDisbetter!",
    // Hypervisors
    Qemu, "TCGTCGTCGTCG",
    Kvm, " KVMKVMKVM  ",
    Vmware, "VMwareVMware",
    VirtualBox, "VBoxVBoxVBox",
    Xen, "XenVMMXenVMM",
    HyperV, "Microsoft Hv"
}

#[derive(Debug)]
pub struct CpuInfo {
    pub vendor: CpuVendor,
    pub x2apic_supported: bool,
    //pub tsc_supported: bool,
    //pub msr_supported: bool,
}

impl Default for CpuInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl CpuInfo {
    // Note: if the computer does not support cpuid, this will generate an invalid opcode fault.
    #[must_use]
    pub fn new() -> Self {
        let ecx = cpuid(1).ecx;
        CpuInfo {
            vendor: get_vendor(),
            x2apic_supported: check_feature(CpuFeature::X2Apic, ecx),
        }
    }
}

/// # Panics
/// This can panic if combine is not a valid UTF-8 sequence.
#[must_use]
pub fn get_vendor() -> CpuVendor {
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

    CpuVendor::from(s)
}

#[must_use]
pub fn check_feature(feature: CpuFeature, bit: u32) -> bool {
    let f = feature as u32;
    (bit & f) != 0
}

fn cpuid(eax: u32) -> CpuidResult {
    unsafe { core::arch::x86_64::__cpuid(eax) }
}
