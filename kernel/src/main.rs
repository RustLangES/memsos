#![no_std]
#![no_main]
#![feature(sync_unsafe_cell)]

mod allocator;
mod mem;
mod requests;
mod writer;

use core::fmt::Write;

use allocator::Allocator;
use limine::BaseRevision;
use limine::request::{RequestsEndMarker, RequestsStartMarker};

use crate::mem::frame::init_frame_allocator;
use crate::requests::HHDM_REQUEST;
use crate::writer::init_writer;

#[global_allocator]
static ALLOCATOR: Allocator = Allocator::new();

extern crate alloc;

#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();
#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    assert!(BASE_REVISION.is_supported());

    ALLOCATOR.init();
    init_writer();

    let hhdm = HHDM_REQUEST
        .get_response()
        .expect("Cannot get HHDM")
        .offset();
    init_frame_allocator(hhdm);

    println!("It works!");

    loop {}
}

#[panic_handler]
fn panic_hnadler(info: &core::panic::PanicInfo) -> ! {
    println!("{:?}", info.message());
    loop {}
}
