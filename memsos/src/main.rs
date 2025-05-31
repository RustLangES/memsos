#![feature(uefi_std)]
use std::os::uefi as uefi_std;
use uefi::runtime::ResetType;
use uefi::{Handle, Status};

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
    println!("Hello, world!");
    uefi::runtime::reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
}
