use crate::once::Once;

pub mod frame;
pub mod paging;

pub const KERNEL_OFFSET: u64 = 0xffff_ffff_8000_0000;
pub static HIGHER_HALF_OFFSET: Once<u64> = Once::new();
