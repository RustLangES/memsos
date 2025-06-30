#![no_std]
#![no_main]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use r_efi::efi;
use core::fmt::Write;

#[global_allocator]
static GLOBAL_ALLOCATOR: r_efi_alloc::global::Bridge = r_efi_alloc::global::Bridge::new();


static mut SYSTEM_TABLE: *mut efi::SystemTable = core::ptr::null_mut();

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
    unsafe {
        //SYSTEM_TABLE = st;
    }
    
    unsafe { 
        let mut allocator = r_efi_alloc::alloc::Allocator::from_system_table(st, efi::LOADER_DATA);
        let _attachment = GLOBAL_ALLOCATOR.attach(&mut allocator);
    }
    
    efi_run(_h, st)
}

fn efi_run(_h: efi::Handle, st: *mut efi::SystemTable) -> efi::Status {
    let s: String;
    let mut v: Vec<u16>;
    s = String::from("Hello World!\n");
    v = s.encode_utf16().collect();
    v.push(0);
    let r =
        unsafe { ((*(*st).con_out).output_string)((*st).con_out, v.as_mut_slice().as_mut_ptr()) };
    if r.is_error() {
        return r;
    }
    let r = unsafe {
        let mut x: usize = 0;
        ((*(*st).boot_services).wait_for_event)(1, &mut (*(*st).con_in).wait_for_key, &mut x)
    };
    if r.is_error() {
        return r;
    }

    efi::Status::SUCCESS
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
