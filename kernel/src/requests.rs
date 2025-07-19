use limine::request::{ExecutableAddressRequest, FramebufferRequest, HhdmRequest};

#[used]
#[unsafe(link_section = ".requests")]
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static KERNEL_ADDRESS: ExecutableAddressRequest = ExecutableAddressRequest::new();
