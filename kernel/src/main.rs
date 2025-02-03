#![no_std]
#![no_main]
#![allow(clippy::similar_names)]
#![feature(sync_unsafe_cell)]

mod asm;
mod drivers;
mod mem;
mod memtest;
mod power;
mod ui;

use bootloader_api::{config::Mapping, entry_point, BootInfo, BootloaderConfig};
use core::panic::PanicInfo;
use power::reboot::reboot;
use ui::widget::input::input;

use ui::{
    layout::{vertical::VerticalLayout, Layout, LayoutParams},
    widget::line::line,
    writer::{clear, init_ui},
};

use mem::init_mem;
use memtest::run_test;

const CONFIG: BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();

    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};
entry_point!(kernel_main, config = &CONFIG);

const PADDING: isize = 20;

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let physical = &boot_info.physical_memory_offset.into_option();
    let regions = &boot_info.memory_regions;
    let api_version = &boot_info.api_version;
    let framebuffer = boot_info.framebuffer.take().unwrap();
    let info = framebuffer.info();
    let buffer = framebuffer.into_buffer();

    let Some(mem_offset) = physical else { loop {} };

    init_ui(buffer, info);
    init_mem(*mem_offset);

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

    let info_layout = VerticalLayout::new(LayoutParams {
        padding: 0,
        line_size: Some(640),
        start_pos: (30, 30),
        max_y: None,
    });

    clear();

    render!(
        &line((PADDING, PADDING), (PADDING, h - PADDING)),
        &line((PADDING, h - PADDING), (w - PADDING, h - PADDING)),
        &line((w - PADDING, PADDING), (w - PADDING, h - PADDING)),
        &line((PADDING, PADDING), (w - PADDING, PADDING)),
        &line((PADDING, h / 2), (w - PADDING, h / 2)),
        &line((w / 2, PADDING), (w / 2, h / 2))
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

    for region in regions.iter() {
        run_test(&debug_layout, *region);
    }
   
    layout!(
        debug_layout,
        &text!("Test completed")
    );

    loop {}
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
