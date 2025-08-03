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

        impl From<&'static str> for CpuVendor {
            fn from(value: &'static str) -> Self {
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

pub fn check_feature(feature: CpuFeature, val: u32) -> bool {
    let f = feature as u32;
    (val & f) != 0
}

fn cpuid(eax: u32) -> CpuidResult {
    unsafe { core::arch::x86_64::__cpuid(eax) }
}
