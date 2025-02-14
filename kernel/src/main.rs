#![no_std]
#![no_main]

use bootloader_api::{
    config::Mapping, entry_point, info::MemoryRegionKind, BootInfo, BootloaderConfig, info::MemoryRegions
};
use core::panic::PanicInfo;
use heapless::String;
use os::{
    arch::{cpuid::CpuInfo, reboot::reboot},
    mem::MemWriter,
    ui::{
        layout::{vertical::VerticalLayout, Layout, LayoutParams},
        logger::DebugLogger,
        widget::{ask::Ask, input::input, line::line, text::TextStyle},
        writer::{clear, init_ui},
    },
    PADDING,
};
use os::{ask, layout, render, styled_text, text};

use memsos_core::{run_test, MemoryRegion, TestResult};

const CONFIG: BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();

    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};
entry_point!(kernel_main, config = &CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let physical = &boot_info.physical_memory_offset.into_option();
    let regions = &boot_info.memory_regions;
    let api_version = &boot_info.api_version;
    let framebuffer = boot_info.framebuffer.take().unwrap();
    let info = framebuffer.info();
    let buffer = framebuffer.into_buffer();

    let Some(mem_offset) = physical else {
        panic!("no physical memory")
    };

    let memory_writer = MemWriter::create(*mem_offset);

    init_ui(buffer, info);

    let memsos_version = env!("CARGO_PKG_VERSION");
    let h: isize = info.height.try_into().unwrap();
    let w: isize = info.width.try_into().unwrap();

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
        (info.width - (info.width / 2) + 6, 30),
        os::ui::widget::text::TextStyle { invert: true },
        "Memtest Info"
    );

    let test_info_layout = VerticalLayout::new(LayoutParams {
        padding: 0,
        line_size: None,
        start_pos: (info.width - (info.width / 2) + 6, 70),
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
        &text!((0,0),"Mem Size: {:.2} GB", calculate_total_memory_gb(regions))
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
        &text!(
            (0, 0),
            "bootloader v{}.{}.{}",
            api_version.version_major(),
            api_version.version_minor(),
            api_version.version_patch()
        ),
        &text!((0, 0), "Mem regions: {:?}", regions),
        &text!("Made with love by Rust Lang Es")
    );

    let mut test_result = TestResult::default();

    for region in regions.iter() {
        if region.kind != MemoryRegionKind::Usable {
            layout!(
                &debug_layout,
                &text!((0, 0), "Omitting region of memory {:?}", region)
            );
            continue;
        }
        test_result += run_test(
            &mut logger,
            &memory_writer,
            &MemoryRegion {
                start: region.start,
                end: region.end,
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

fn calculate_total_memory_gb(regions: &MemoryRegions) -> f64 {
    let mut total_memory_kb = 0;

    for region in regions.iter() {
        let region_size_kb = (region.end - region.start + 1) / 1024;
        total_memory_kb += region_size_kb;
    }

    let total_memory_gb = total_memory_kb as f64 / 1048576.0;
    

    total_memory_gb
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
