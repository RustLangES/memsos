#![no_std]
#![feature(sync_unsafe_cell)]

pub mod cpuid;
pub mod hcf;
pub mod interrupt;
pub mod msr;

pub mod speaker;
pub use msr::rdmsr;
