#![no_std]
#![feature(sync_unsafe_cell)]

pub mod frame;
pub mod linked_list;

extern crate alloc;

use linked_list::Allocator;

#[global_allocator]
pub static ALLOCATOR: Allocator = Allocator::new();
