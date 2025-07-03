#![no_std]
#![no_main]
#![feature(fn_traits)]

extern crate alloc;
mod once;

use alloc::{string::String, vec::Vec};
use r_efi::{efi::{self, Char16}, protocols::graphics_output::BltPixel};
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


// https://github.com/r-efi/r-efi/blob/main/examples/gop-query.rs
fn locate_singleton(
    st: *mut efi::SystemTable,
    guid: *const efi::Guid,
) -> Result<*mut core::ffi::c_void, efi::Status> {
    let mut interface: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut handles: *mut efi::Handle = core::ptr::null_mut();
    let mut n_handles: usize = 0;
    let mut r: efi::Status;

    unsafe {
        if (*st).hdr.revision < efi::SYSTEM_TABLE_REVISION_1_10 {
            return Err(efi::Status::UNSUPPORTED);
        }

        let r = ((*(*st).boot_services).locate_handle_buffer)(
            efi::BY_PROTOCOL,
            guid as *mut _,
            core::ptr::null_mut(),
            &mut n_handles,
            &mut handles,
        );
        match r {
            efi::Status::SUCCESS => {}
            efi::Status::NOT_FOUND => return Err(r),
            efi::Status::OUT_OF_RESOURCES => return Err(r),
            _ => panic!(),
        };
    }

    unsafe {
        r = efi::Status::NOT_FOUND;
        for i in 0..n_handles {
            r = ((*(*st).boot_services).handle_protocol)(
                *handles.offset(core::convert::TryFrom::<usize>::try_from(i).unwrap()),
                guid as *mut _,
                &mut interface,
            );
            match r {
                efi::Status::SUCCESS => break,
                efi::Status::UNSUPPORTED => continue,
                _ => panic!(),
            };
        }
    }

    unsafe {
        let r = ((*(*st).boot_services).free_pool)(handles as *mut core::ffi::c_void);
        assert!(!r.is_error());
    }
    match r {
        efi::Status::SUCCESS => Ok(interface),
        _ => Err(efi::Status::NOT_FOUND),
    }
}

fn query_gop(
    gop: *mut efi::protocols::graphics_output::Protocol,
) -> Result<(u32, u32), efi::Status> {
    let mut info: *mut efi::protocols::graphics_output::ModeInformation = core::ptr::null_mut();
    let mut z_info: usize = 0;

    unsafe {
        let r = ((*gop).query_mode)(gop, (*(*gop).mode).mode, &mut z_info, &mut info);
        match r {
            efi::Status::SUCCESS => {}
            efi::Status::DEVICE_ERROR => return Err(r),
            _ => panic!(),
        };
        if z_info < core::mem::size_of_val(&*info) {
            return Err(efi::Status::UNSUPPORTED);
        }

        Ok(((*info).horizontal_resolution, (*info).vertical_resolution))
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
    let gop = locate_singleton(st, &efi::protocols::graphics_output::PROTOCOL_GUID).unwrap() as *mut efi::protocols::graphics_output::Protocol;
    let mode = unsafe { *((*gop).mode) };
    let framebuffer =  unsafe { core::slice::from_raw_parts_mut(mode.frame_buffer_base as *mut u8, mode.frame_buffer_size) };
    framebuffer.fill(0xFF);

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
