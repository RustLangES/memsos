#![feature(uefi_std)]

slint::include_modules!();

mod platform;

use arch::protocols::UefiProtocols;
use std::os::uefi as uefi_std;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::runtime::ResetType;
use uefi::{boot, Handle, Status};

use crate::platform::Platform;

fn setup() {
    let st = uefi_std::env::system_table();
    let ih = uefi_std::env::image_handle();

    unsafe {
        uefi::table::set_system_table(st.as_ptr().cast());

        let ih = Handle::from_ptr(ih.as_ptr().cast()).unwrap();
        uefi::boot::set_image_handle(ih);
    }
}

fn main() {
    setup();
    slint::platform::set_platform(Box::<Platform>::default()).unwrap();

    let ui = Ui::new().unwrap();

    ui.run().unwrap();

    uefi::runtime::reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
}
