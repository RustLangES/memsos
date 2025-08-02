use sync::Once;

pub mod allocator;
pub mod frame;
pub mod paging;

pub const KERNEL_OFFSET: u64 = 0xffff_ffff_8000_0000;
