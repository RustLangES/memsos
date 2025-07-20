#![no_std]
pub mod requests;
use sync::Once;

pub static HIGHER_HALF_OFFSET: Once<u64> = Once::new();
