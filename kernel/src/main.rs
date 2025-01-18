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

use bootloader_api::{
    config::Mapping, entry_point, info::MemoryRegionKind, BootInfo, BootloaderConfig,
};
use core::{fmt::Write, panic::PanicInfo};
use drivers::keyboard::{Key, Keyboard};
use memtest::test_memory;
use power::reboot::reboot;

use ui::{
    layout::{vertical::VerticalLayout, Layout},
    widget::line::line,
    writer::{clear, init_ui},
};

const CONFIG: BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();

    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};
entry_point!(kernel_main, config = &CONFIG);

const PADDING: isize = 20;

static INFO_LAYOUT: VerticalLayout = VerticalLayout::new((30, 30), 0);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let physical = &boot_info.physical_memory_offset.into_option();
    let regions = &boot_info.memory_regions;
    let api_version = &boot_info.api_version;
    let framebuffer = boot_info.framebuffer.take().unwrap();
    let info = framebuffer.info();
    let buffer = framebuffer.into_buffer();
    let memsos_version = env!("CARGO_PKG_VERSION");

    init_ui(buffer, info);

    let h: isize = info.height.try_into().unwrap();
    let w: isize = info.width.try_into().unwrap();

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
        &text!("memsos v{memsos_version}"),
        INFO_LAYOUT
    );

    layout!(
        &text!((0, 0), "bootloader v{}.{}.{}", api_version.version_major(), api_version.version_minor(), api_version.version_patch()),
        INFO_LAYOUT
    );

    layout!(
        &text!((0, 0), "Mem Regions: {:?}", regions),
        INFO_LAYOUT
    );

    layout!(
        &text!("Made with love by Rust Lang Es"),
        INFO_LAYOUT
    );

    loop {}
}

#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    clear();

    let text = text!((0, 0), "{:?}", panic);
    render!(&text);

    let keyboard = Keyboard;

    keyboard.wait_key(Key::Space);

    reboot();
    loop {}
}
