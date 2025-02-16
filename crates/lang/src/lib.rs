#![no_std]

#[path = "lang_info.rs"]
mod lang_info;

pub const DIALOGS: lang_info::Dialogs = lang_info::DIALOGS;
