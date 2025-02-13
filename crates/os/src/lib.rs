#![no_std]
#![no_main]
#![allow(clippy::similar_names)]
#![feature(sync_unsafe_cell)]

pub mod arch;
pub mod asm;
pub mod drivers;
pub mod mem;
pub mod ui;

pub const PADDING: isize = 20;
