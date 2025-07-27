#![no_std]

pub mod hcf;
pub mod interrupt;
pub mod msr;
pub mod nmi;
pub mod rtc;
pub mod speaker;
pub mod tsc;
pub use msr::rdmsr;
