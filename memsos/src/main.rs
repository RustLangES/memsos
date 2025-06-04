#![feature(uefi_std)]
use arch::protocols::UefiProtocols;
use std::os::uefi as uefi_std;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::runtime::ResetType;
use uefi::{boot, Handle, Status};

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
    let protocols = UefiProtocols::get();
    let mut gop = protocols.gop;
    let mut fb = gop.frame_buffer();

    for i in 0..fb.size() {
        unsafe {
            fb.write_byte(i, 255);
        }
    }

    uefi::runtime::reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
}
