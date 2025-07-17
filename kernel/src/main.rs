#![no_std]
#![no_main]

mod allocator;
mod requests;
mod writer;
use core::fmt::Write;

use allocator::Allocator;
use limine::BaseRevision;
use limine::request::{RequestsEndMarker, RequestsStartMarker};

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
    ALLOCATOR.init();
    init_writer();

    println!("It works!");

    loop {}
}

#[panic_handler]
fn panic_hnadler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
