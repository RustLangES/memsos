#![no_std]

#[path = "lang_info.rs"]
#[rustfmt::skip]
#[allow(dead_code)]
mod lang_info;

pub use lang_info::DIALOGS;
