#![no_std]
#![no_main]
#![feature(sync_unsafe_cell, fn_traits)]

mod mem;
mod once;
mod requests;
mod writer;

use core::fmt::Write;

use limine::BaseRevision;
use limine::request::{RequestsEndMarker, RequestsStartMarker};
use mem::allocator::Allocator;

use crate::mem::HIGHER_HALF_OFFSET;
use crate::mem::frame::init_frame_allocator;
use crate::mem::paging::{get_kernel_map, init_page_map};
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
extern "C" fn kmain() -> ! {
    assert!(BASE_REVISION.is_supported());

    ALLOCATOR.init();
    init_writer();

    let hhdm = HHDM_REQUEST
        .get_response()
        .expect("Cannot get HHDM")
        .offset();

    HIGHER_HALF_OFFSET.call_once(|| hhdm);

    init_frame_allocator(0x1000);
    init_page_map();

    get_kernel_map().kernel_map();

    println!("It works!");

    #[allow(clippy::empty_loop)]
    loop {}
}

#[panic_handler]
fn panic_hnadler(info: &core::panic::PanicInfo) -> ! {
    println!("{:?}", info.message());
    loop {}
}
