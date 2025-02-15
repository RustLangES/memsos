#![no_std]
#![no_main]

use core::panic::PanicInfo;
use heapless::String;
use limine::memory_map::{Entry, EntryType};
use memsos_core::{run_test, MemoryRegion, TestResult};
use os::boot::BootInfo;
use os::{
    arch::{cpuid::CpuInfo, reboot::reboot},
    mem::MemWriter,
    ui::{
        layout::{vertical::VerticalLayout, Layout, LayoutParams},
        logger::DebugLogger,
        widget::{ask::Ask, input::input, line::line, text::TextStyle},
        writer::{clear, height, init_ui, width},
    },
    PADDING,
};
use os::{ask, layout, render, styled_text, text};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let boot_info = BootInfo::get();
    let mem_offset = &boot_info.offset;
    let regions = &boot_info.memory_regions;

    let limine_info = &boot_info.info;

    let memory_writer = MemWriter::create(*mem_offset);

    init_ui();

    let memsos_version = env!("CARGO_PKG_VERSION");
    let h: isize = height().try_into().unwrap();
    let w: isize = width().try_into().unwrap();

    let debug_layout = VerticalLayout::new(LayoutParams {
        padding: 0,
        start_pos: (
            (PADDING + 2).try_into().unwrap(),
            (h - (h / 2) + 4).try_into().unwrap(),
        ),
        line_size: Some((w - 2).try_into().unwrap()),
        max_y: Some((h - PADDING).try_into().unwrap()),
    });

    let mut logger = DebugLogger::new(&debug_layout);

    let info_layout = VerticalLayout::new(LayoutParams {
        padding: 0,
        line_size: Some(640),
        start_pos: (30, 30),
        max_y: None,
    });

    let memtest_message = styled_text!(
        (width() - (width() / 2) + 6, 30),
        TextStyle { invert: true },
        "Memtest Info"
    );

    let test_info_layout = VerticalLayout::new(LayoutParams {
        padding: 0,
        line_size: None,
        start_pos: (width() - (width() / 2) + 6, 70),
        max_y: None,
    });

    let cpuinfo = CpuInfo::new();

    let question = ask!("basic", "advanced");

    clear();

    render!(&question);

    clear();
    let response = memsos_core::MemTestKind::try_from(question.get_result()).unwrap();

    render!(
        &line((PADDING, PADDING), (PADDING, h - PADDING)),
        &line((PADDING, h - PADDING), (w - PADDING, h - PADDING)),
        &line((w - PADDING, PADDING), (w - PADDING, h - PADDING)),
        &line((PADDING, PADDING), (w - PADDING, PADDING)),
        &line((PADDING, h / 2), (w - PADDING, h / 2)),
        &line((w / 2, PADDING), (w / 2, h / 2))
    );

    render!(&memtest_message);

    layout!(
        test_info_layout,
        &styled_text!((0, 0), TextStyle { invert: true }, "Mem Info"),
        &text!(
            (0, 0),
            "Mem Size: {:.2} GB",
            calculate_total_memory_gb(regions)
        ),
        &text!("Mem Speed: Faied to load Information")
    );

    layout!(
        test_info_layout,
        &styled_text!((0, 0), TextStyle { invert: true }, "Cpu info"),
        &text!((0, 0), "Kind of test: {:?}", response),
        &text!((0, 0), "Model: {}", cpuinfo.model),
        &text!((0, 0), "Vendor: {:?}", cpuinfo.vendor),
        &text!((0, 0), "family: {}", cpuinfo.family),
        &text!((0, 0), "Stepping: {}", cpuinfo.stepping)
    );

    layout!(
        info_layout,
        &text!("memsos v{memsos_version}"),
        &text!((0, 0), "limine version {}", limine_info.version()),
        &text!((0, 0), "bootloader v{}", boot_info.info.version(),),
        &text!("Made with love by RustLangEs (Rust Lang en Español)")
    );

    let mut test_result = TestResult::default();

    for region in regions.iter() {
        if region.entry_type != EntryType::USABLE {
            layout!(
                &debug_layout,
                &text!(
                    (0, 0),
                    "Omitting region of memory {}-{}",
                    region.base,
                    region.base + region.length
                )
            );
            continue;
        }
        test_result += run_test(
            &mut logger,
            &memory_writer,
            &MemoryRegion {
                start: region.base,
                end: region.base + region.length,
            },
            response,
        );
    }

    layout!(
        &test_info_layout,
        &styled_text!((0, 0), TextStyle { invert: true }, "Test result"),
        &text!("Test Completed..."),
        &text!(
            (0, 0),
            "Number of faulty memory addrs {}",
            test_result.bad_addrs
        )
    );

    #[allow(clippy::empty_loop)]
    loop {}
}

fn calculate_total_memory_gb(regions: &[&Entry]) -> f64 {
    let mut total_memory_kb = 0;

    for region in regions.iter() {
        if region.entry_type == EntryType::USABLE
            || region.entry_type == EntryType::BOOTLOADER_RECLAIMABLE
            || region.entry_type == EntryType::KERNEL_AND_MODULES
            || region.entry_type == EntryType::ACPI_RECLAIMABLE
        {
            let region_size_kb = ((region.base + region.length) - region.base + 1) / 1024;
            total_memory_kb += region_size_kb;
        }
    }

    total_memory_kb as f64 / 1048576.0
}

#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    clear();

    let panic_layout = VerticalLayout::new(LayoutParams {
        start_pos: (0, 0),
        padding: 0,
        line_size: None,
        max_y: None,
    });

    layout!(
        &panic_layout,
        &text!((0, 0), "Panic! {}", panic.message()),
        &input(&text!((0, 10), "Press space to reboot your computer"))
    );

    reboot();

    loop {}
}
