#![no_std]
#![no_main]

use core::panic::PanicInfo;
use lang::DIALOGS;
use limine::memory_map::{Entry, EntryType};
use memsos_core::{run_test, MemoryRegion, TestResult};

use os::boot::BootInfo;
use os::{
    arch::{cpuid::CpuInfo, reboot::reboot},
    mem::MemWriter,
    ui::{
        layout::{vertical::VerticalLayout, Layout, LayoutParams},
        logger::DebugLogger,
        widget::{ask::ask, input::input, line::line, text::TextStyle},
        writer::{clear, height, init_ui, width},
    },
    PADDING,
};
use os::{layout, render, styled_text, text};

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
        "{}",
        DIALOGS.memtest_info.info
    );

    let test_info_layout = VerticalLayout::new(LayoutParams {
        padding: 0,
        line_size: None,
        start_pos: (width() - (width() / 2) + 6, 70),
        max_y: None,
    });

    let cpuinfo = CpuInfo::new();

    let question = ask(&[DIALOGS.ask.basic, DIALOGS.ask.advanced]);

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
        &text!((0, 0), "{}: {}", DIALOGS.memtest_info.kind_test, response),
        &styled_text!(
            (0, 0),
            TextStyle { invert: true },
            "{}",
            DIALOGS.mem_info.info
        ),
        &text!(
            (0, 0),
            "{} {:.2} GB",
            DIALOGS.mem_info.size,
            calculate_total_memory_gb(regions),
        ),
        &text!("TODO: Mem Speed")
    );

    layout!(
        test_info_layout,
        &styled_text!(
            (0, 0),
            TextStyle { invert: true },
            "{}",
            DIALOGS.cpu_info.info
        ),
        &text!((0, 0), "{}: {}", DIALOGS.cpu_info.model, cpuinfo.model),
        &text!((0, 0), "{}: {:?}", DIALOGS.cpu_info.vendor, cpuinfo.vendor),
        &text!((0, 0), "{}: {}", DIALOGS.cpu_info.family, cpuinfo.family),
        &text!(
            (0, 0),
            "{}: {}",
            DIALOGS.cpu_info.stepping,
            cpuinfo.stepping
        )
    );

    #[cfg(target_arch = "x86_64")]
    {
        let smbios = os::arch::smbios::read_smbios();
        if let Ok(entry) = smbios {
            layout!(
                test_info_layout,
                &text!((0, 0), "SMBIOS: {:?}", unsafe { *entry }.anchor)
            );
        } else {
            layout!(test_info_layout, &text!("Smbios not detected"));
        }
    }

    layout!(
        info_layout,
        &text!("memsos v{memsos_version}"),
        &text!(
            (0, 0),
            "{} {}",
            DIALOGS.info.bootloader_version,
            limine_info.version()
        ),
        &text!((0, 0), "{}", DIALOGS.info.love_message)
    );

    let mut test_result = TestResult::default();

    for region in regions.iter() {
        if region.entry_type != EntryType::USABLE {
            layout!(
                &debug_layout,
                &text!(
                    (0, 0),
                    "{} {}-{}",
                    DIALOGS.debug_info.omitting,
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
        &styled_text!(
            (0, 0),
            TextStyle { invert: true },
            "{}",
            DIALOGS.test_result_info.info
        ),
        &text!((0, 0), "{}", DIALOGS.test_result_info.completed_message),
        &text!(
            (0, 0),
            "{} {}",
            DIALOGS.test_result_info.number_of_errors,
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
