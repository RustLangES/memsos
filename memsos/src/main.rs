#![no_std]
#![no_main]
#![feature(fn_traits)]

extern crate alloc;
mod once;

use alloc::{string::String, vec::Vec};
use r_efi::efi::{self, Char16};
use core::fmt::Write;
use once::Once;

#[global_allocator]
static GLOBAL_ALLOCATOR: r_efi_alloc::global::Bridge = r_efi_alloc::global::Bridge::new();

static SYSTEM_TABLE: Once<*mut efi::SystemTable> = Once::new();

const EFI_BLACK: usize = 0x0;
const EFI_RED: usize = 0x04;
const EFI_WHITE: usize = 0x0F;

macro_rules! print {
    ($($arg:tt)*) => {
        let args = format_args!($($arg)*);
        let mut buffer = String::new();
        buffer.clear();
        write!(buffer, "{}", args).unwrap();
        let mut v: alloc::vec::Vec<u16> = buffer.encode_utf16().collect();
        v.push(0);

         let r =
        unsafe { ((*(*$crate::SYSTEM_TABLE).con_out).output_string)((*$crate::SYSTEM_TABLE).con_out, v.as_mut_slice().as_mut_ptr()) };
    if r.is_error() {
        //return r;
    }
    }
}

macro_rules! println {
    ($($arg:tt)*) => {
        let args = format_args!($($arg)*);
        
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn efi_main(_h: efi::Handle, st: *mut efi::SystemTable) -> efi::Status {
    SYSTEM_TABLE.call_once(|| st);
    
    unsafe { 
        let mut allocator = r_efi_alloc::alloc::Allocator::from_system_table(st, efi::LOADER_DATA);
        let _attachment = GLOBAL_ALLOCATOR.attach(&mut allocator);
    }
    
    efi_run(_h, st)
}

fn efi_run(_h: efi::Handle, st: *mut efi::SystemTable) -> efi::Status {

    let s = String::from("Hello from memsos!");
    
    let mut v: Vec<u16> = s.encode_utf16().collect();
    v.push(0);
    unsafe {
        ((*(*st).con_out).set_attribute)((*st).con_out, EFI_WHITE | (EFI_RED << 4));

        ((*(*st).con_out).clear_screen)((*st).con_out);
         
//        panic!();
//
    }

    loop {}

    efi::Status::SUCCESS
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    let st = *SYSTEM_TABLE;

    unsafe {
        ((*(*st).runtime_services).reset_system)(efi::RESET_COLD, efi::Status::ABORTED, 0, core::ptr::null_mut());

    }
    loop {}
}
