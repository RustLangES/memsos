#![no_std]
#![no_main]
#![feature(sync_unsafe_cell, fn_traits, abi_x86_interrupt)]
#![allow(unused_imports, dead_code)]

extern crate alloc;

mod idt;

use acpi::{ACPI_TABLE, init_acpi};
use alloc::vec::Vec;
use allocators::ALLOCATOR;
use arch::cpuid::CpuInfo;
use arch::hcf::hcf;
use arch::speaker::beep;
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
use modulo_n::ModuloN;
use pci::io::xhci_scan;
use sync::Once;
use timers::rtc::restore_rtc;
use timers::tsc::{Instant, TSC_TICKS_PER_MS, calibrate_tsc, rdtsc, sleep};
use ui::sections::cpu_info::CpuInfoSection;
use ui::sections::loading::LoadingSection;
use x86_64::instructions::port::Port;
use x86_64::structures::paging::page_table::PageTableEntry;
use x86_64::structures::paging::{PageTableFlags, PageTableIndex};

use ui::sections::test_info::TestInfoSection;
use ui::{
    RenderSection, Section, UiState, get_ui_state, init_ui_state, push_logs, render_section,
    render_ui_state,
};

use x86_64::{PhysAddr, VirtAddr};

use crate::idt::{TIMER_VECTOR, init_idt};
use allocators::frame::init_frame_allocator;
use paging::{get_page_table, init_page_map, map_range, umap_range};
use x2apic::{X2APIC, X2Apic, init_x2apic};

use boot::HIGHER_HALF_OFFSET;
use boot::requests::HHDM_REQUEST;
use fb::init_writer;

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

    get_ui_writer().clear(Rgb888::new(11, 11, 10)).unwrap();

    let mem_map = &boot::requests::MEMORY_MAP_REQUEST;
    let entries = mem_map.get_response().unwrap().entries();

    init_frame_allocator(0x2000);
    init_page_map();

    init_idt();

    let mut loading = LoadingSection::new(Point::zero());
    loading.render(get_ui_writer());

    let acpi_result = init_acpi();
    if let Err(ref e) = acpi_result {
        println!("Acpi Error: {e}");
    }

    let (_power_profile, oem_id) =
        acpi_result.unwrap_or((0, "NOACPI".as_bytes().try_into().unwrap()));

    println!("{}", ACPI_TABLE.has_value());

    println!("{:#?}", xhci_scan());

    calibrate_tsc();
    init_mem_module(entries);
    init_x2apic();

    println!("{}", TSC_TICKS_PER_MS.has_value());

    init_ui_state(UiState {
        test_info_section: TestInfoSection::new(Point::new(30, 30)),
        cpu_info_section: CpuInfoSection::new(oem_id, Point { x: 0, y: 0 }),
    });

    get_ui_writer().clear(Rgb888::BLACK).unwrap();

    render_ui_state();

    let table = get_page_table(VirtAddr::new(*HIGHER_HALF_OFFSET));
    let index = PageTableIndex::new(((*HIGHER_HALF_OFFSET >> 39) as u16) & 0x1FF);
    let entry_pml4 = &mut table[index];

    X2APIC.oneshot(TIMER_VECTOR, Duration::from_secs(1));

    let mut reports = Vec::new();

    let flags_pml4 = entry_pml4.flags();

    let entry_pdpt = unsafe {
        &mut *((entry_pml4.addr().as_u64() + *HIGHER_HALF_OFFSET) as *mut PageTableEntry)
    };

    let flags_pdpt = entry_pdpt.flags();

    let entry_pd = unsafe {
        &mut *((entry_pdpt.addr().as_u64() + *HIGHER_HALF_OFFSET) as *mut PageTableEntry)
    };

    let flags_pd = entry_pd.flags();

    let entry_pt =
        unsafe { &mut *((entry_pd.addr().as_u64() + *HIGHER_HALF_OFFSET) as *mut PageTableEntry) };

    let flags_pt = entry_pt.flags();

    let flags = PageTableFlags::WRITABLE | PageTableFlags::PRESENT | PageTableFlags::NO_CACHE;
    entry_pml4.set_flags(flags);
    entry_pdpt.set_flags(flags);
    entry_pd.set_flags(flags);
    entry_pt.set_flags(flags);

    unsafe {
        core::arch::asm!("invlpg [{0}]", in(reg) *HIGHER_HALF_OFFSET);
    }

    unsafe {
        Port::<u32>::new(0x60).write(0xF5);

        Port::<u32>::new(0x64).write(0xD4);

        Port::<u32>::new(0x60).write(0xF4);

        let mut mouse_x: u64 = 0;
        let mut mouse_y: u64 = 0;

        loop {
            Port::<u32>::new(0x64).write(0xD4);
            let status = Port::<u32>::new(0x64).read();
            if (status & 1) != 0 {
                let writer = crate::get_fb_writer();
                //writer.clear();
                let data = Port::<u32>::new(0x60).read();

                let first_byte = data & 0xFF;
                let second_byte = ((data >> 8) & 0xFF) as u8;
                let third_byte = ((data >> 16) & 0xFF) as u8;

                let state = first_byte as u16;

                let rel_x: i64 = i64::from(second_byte.cast_signed());

                let rel_y: i64 = i64::from(third_byte.cast_signed());

                mouse_x = mouse_x.saturating_add_signed(rel_x);
                mouse_y = mouse_y.saturating_sub_signed(rel_y);

                if mouse_x < writer.width() as u64 && mouse_y < writer.height() as u64 {
                    writer.write_pixel(mouse_x, mouse_y, 0xffff_ffff);
                }

                writer.x = 0;
                writer.y = 0;
                write!(writer, "{}\n", mouse_x);
                write!(writer, "{}\n", mouse_y);
            }
        }
    }

    panic!("no");

    load_memtest::<MarchC>(&mut reports);
    load_memtest::<ModuloN>(&mut reports);
    get_ui_state().test_info_section.time.enabled = false;

    entry_pml4.set_flags(flags_pml4);
    entry_pdpt.set_flags(flags_pdpt);
    entry_pd.set_flags(flags_pd);
    entry_pt.set_flags(flags_pt);

    unsafe {
        core::arch::asm!("invlpg [{0}]", in(reg) *HIGHER_HALF_OFFSET);
    }

    beep();

    color_print!(Rgb888::CYAN, "It works! {}", reports.is_empty());

    hcf();
}

// TODO: improve panic handler
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    println!("{:?}\n{:?}", info.message(), info.location());
    loop {}
}
