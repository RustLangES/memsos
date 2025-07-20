#![no_std]
#![no_main]
#![feature(sync_unsafe_cell, fn_traits)]

mod mem;
use alloc::vec::Vec;
use commons::mem::{init_mem_module, load_memtest};
use core::fmt::Write;
use fb::println;
use limine::BaseRevision;
use limine::request::{RequestsEndMarker, RequestsStartMarker};
use march_c::MarchC;
use mem::allocator::Allocator;

use crate::mem::frame::init_frame_allocator;
use crate::mem::paging::{get_kernel_map, init_page_map};
use boot::HIGHER_HALF_OFFSET;
use boot::requests::HHDM_REQUEST;
use fb::init_writer;

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

    let mem_map = &boot::requests::MEMORY_MAP_REQUEST;
    let entries = mem_map.get_response().unwrap().entries();
    let mut reports = Vec::new();

    init_mem_module(entries);

    load_memtest::<MarchC>(&mut reports);

    println!("It works!");

    #[allow(clippy::empty_loop)]
    loop {}
}

#[panic_handler]
fn panic_hnadler(info: &core::panic::PanicInfo) -> ! {
    println!("{:?}", info.message());
    loop {}
}
