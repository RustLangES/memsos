#![no_std]
#![no_main]
#![feature(sync_unsafe_cell, fn_traits, abi_x86_interrupt)]
#![allow(unused_imports, dead_code)]

mod idt;
mod mem;

use alloc::vec::Vec;
use arch::cpuid::CpuInfo;
use arch::hcf::hcf;
use arch::rtc::restore_rtc;
use arch::speaker::beep;
use arch::tsc::{Instant, TSC_TICKS_PER_MS, calibrate_tsc, rdtsc, sleep};
use bit_fade::BitFade;
use commons::mem::{MemoryError, MemoryReport, init_mem_module, load_memtest};
use core::f32;
use core::fmt::Write;
use core::time::Duration;
use embedded_graphics::Drawable;
use embedded_graphics::image::Image;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::iso_8859_9::FONT_6X10;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::{DrawTarget, OriginDimensions, Point, RgbColor};
use embedded_graphics::primitives::{PrimitiveStyle, StyledDrawable};
use embedded_graphics::text::Text;
use fb::{color_print, get_fb_writer, get_ui_writer, init_ui, println};
use limine::BaseRevision;
use limine::request::{RequestsEndMarker, RequestsStartMarker};
use march_c::MarchC;
use mem::allocator::Allocator;
use modulo_n::ModuloN;
use sync::Once;
use ui::components::image::UiImage;

use ui::sections::test_info::TestInfoSection;
use ui::{RenderSection, UiState, get_ui_state, init_ui_state, render_section, render_ui_state};

use x86_64::VirtAddr;

use crate::idt::{TIMER_VECTOR, init_idt};
use crate::mem::frame::init_frame_allocator;
use crate::mem::paging::init_page_map;
use x2apic::{X2APIC, X2Apic, init_x2apic};

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

/// TODO: save the state of the tsc and restore it at the end
#[unsafe(no_mangle)]
extern "C" fn kmain() -> ! {
    assert!(BASE_REVISION.is_supported());

    ALLOCATOR.init();
    init_writer();
    init_ui();

    let hhdm = HHDM_REQUEST
        .get_response()
        .expect("Cannot get HHDM")
        .offset();

    HIGHER_HALF_OFFSET.call_once(|| hhdm);

    let mem_map = &boot::requests::MEMORY_MAP_REQUEST;
    let entries = mem_map.get_response().unwrap().entries();

    init_frame_allocator(0x2000);
    init_page_map();

    init_idt();

    println!("Starting memsos");
    println!("Calibrating tsc...");
    calibrate_tsc();
    restore_rtc();
    init_mem_module(entries);
    init_x2apic();
    init_ui_state(UiState {
        test_info_section: TestInfoSection::new(Point::new(0, 0)),
    });

    let mut reports = Vec::new();

    get_ui_writer().clear(Rgb888::BLACK).unwrap();

    render_ui_state();

    X2APIC.oneshot(TIMER_VECTOR, Duration::from_secs(1));

    load_memtest::<MarchC>(&mut reports);
    load_memtest::<ModuloN>(&mut reports);
    get_ui_state().test_info_section.time.enabled = false;

    beep();

    color_print!(Rgb888::CYAN, "It works! {}", reports.is_empty());
    println!("asd");

    hcf();
}

// TODO: improve panic handler
#[panic_handler]
fn panic_hnadler(info: &core::panic::PanicInfo) -> ! {
    println!("{:?}\n{:?}", info.message(), info.location());
    loop {}
}
