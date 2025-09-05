#![no_std]
#![feature(sync_unsafe_cell)]

pub mod cpuid;
pub mod hcf;
pub mod interrupt;
pub mod msr;
pub mod nmi;
pub mod paging;
pub mod rtc;
pub mod speaker;
pub mod tsc;
pub use msr::rdmsr;
