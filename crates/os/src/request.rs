use limine::request::{
    BootloaderInfoRequest, FramebufferRequest, HhdmRequest, MemoryMapRequest, SmbiosRequest,
};

#[used]
#[link_section = ".requests"]
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[link_section = ".requests"]
pub static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

#[used]
#[link_section = ".requests"]
pub static SMBIOS_REQUEST: SmbiosRequest = SmbiosRequest::new();

#[used]
#[link_section = ".requests"]
pub static BOOT_INFO_REQUEST: BootloaderInfoRequest = BootloaderInfoRequest::new();

#[used]
#[link_section = ".request"]
pub static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();
