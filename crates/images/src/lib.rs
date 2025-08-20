#![no_std]

use core::include_bytes;

pub static LOGO_CONTENT: &[u8] = include_bytes!("../static/logo.tga");
