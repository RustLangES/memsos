#![no_std]
#![no_main]
#![feature(fn_traits, sync_unsafe_cell)]

//TODO: abstract this in a workspace in a nicer way, for the moment as we are just starting I guess it's ok.

extern crate alloc;
mod fb;
mod bump;
mod once;

use alloc::{fmt::format, string::String, vec::Vec};
use core::{alloc::GlobalAlloc, cell::SyncUnsafeCell, ffi::c_void, fmt::Write};
use once::Once;
use r_efi::{
    efi::{self, Char16, MemoryDescriptor, MemoryType, Status},
    protocols::graphics_output::{
        BltPixel, GraphicsPixelFormat, ModeInformation,
        PIXEL_BLUE_GREEN_RED_RESERVED_8_BIT_PER_COLOR,
        PIXEL_RED_GREEN_BLUE_RESERVED_8_BIT_PER_COLOR,
    },
};

use crate::{bump::BumpAllocator, fb::{Framebuffer, TextRender}};

#[global_allocator]
static mut BUMP_ALLOCATOR: BumpAllocator = BumpAllocator::new();

static SYSTEM_TABLE: Once<*mut efi::SystemTable> = Once::new();

//TODO: Refactor this 

pub static TEXT_OUT: SyncUnsafeCell<Option<TextRender>> = SyncUnsafeCell::new(None);

const EFI_BLACK: usize = 0x0;
const EFI_RED: usize = 0x04;
const EFI_WHITE: usize = 0x0F;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        let args = format_args!($($arg)*);
        let mut buffer = heapless::String::<1024>::new();
        buffer.clear();
        write!(buffer, "{}", args).expect("Cannot format args");
        let writer = unsafe { (*TEXT_OUT.get()).as_mut().unwrap() };

        writer.write_str(buffer.as_str());
    }
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        crate::print!($($arg)*);
        let writer = unsafe { (*TEXT_OUT.get()).as_mut().unwrap() };

        writer.newline();
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
) -> Result<ModeInformation, efi::Status> {
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

        Ok(*info)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn efi_main(_h: efi::Handle, st: *mut efi::SystemTable) -> efi::Status {
    SYSTEM_TABLE.call_once(|| st);

    unsafe {
        let alloc = &raw mut BUMP_ALLOCATOR;
        (*alloc).init(st);
    }

    efi_run(_h, st)
}

fn efi_run(_h: efi::Handle, st: *mut efi::SystemTable) -> efi::Status {
    let gop = locate_singleton(st, &efi::protocols::graphics_output::PROTOCOL_GUID).unwrap()
        as *mut efi::protocols::graphics_output::Protocol;
    let mode = unsafe { *((*gop).mode) };
    let info = query_gop(gop).unwrap();
    let mut fb = Framebuffer {
        version: info.version,
        fb: unsafe {
            core::slice::from_raw_parts_mut(
                mode.frame_buffer_base as *mut u8,
                mode.frame_buffer_size,
            )
        },
        info,
    };

    fb.clear();
    unsafe {
        *TEXT_OUT.get() = Some(TextRender::new(fb));
    }

    let temp = alloc::format!("a{}", "!");
    println!("{}", temp);

    

    loop {}

    efi::Status::SUCCESS
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    let st = *SYSTEM_TABLE;

    
    unsafe {
        let mut text = TEXT_OUT.get().read().unwrap();
        text.clear();
        
        let msg = info.message();
        println!("{:?}", msg);
        
        (&(*(*st).boot_services).stall)(5_000_000);
        ((*(*st).runtime_services).reset_system)(
            efi::RESET_COLD,
            efi::Status::ABORTED,
            0,
            core::ptr::null_mut(),
        );
    }
    loop {}
}
