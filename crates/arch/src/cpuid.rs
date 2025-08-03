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

fn cpuid(eax: u32) -> CpuidResult {
    unsafe { core::arch::x86_64::__cpuid(eax) }
}
