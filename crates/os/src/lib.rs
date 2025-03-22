#![no_std]
#![no_main]
#![allow(clippy::similar_names)]
#![feature(sync_unsafe_cell)]
#![feature(naked_functions)]

pub mod arch;
pub mod asm;
pub mod boot;
pub mod drivers;
pub mod mem;
pub mod request;
pub mod ui;

pub const PADDING: isize = 20;
