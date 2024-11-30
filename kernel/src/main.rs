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

use memtest::test_memory;
use ui::{Color, MemsosUI, MemsosUIWriter, UI};

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

    init_ui!(buffer, info);
    let ui = get_ui!();

    let Some(mem_offset) = physical else { loop {} };

    (*ui).new_row();
    ui.label("Api Info: {:?}");

    (*ui).new_row();
    ui.label("Memsos version: {:?}");
    // (*ui).new_row();
    // TODO: resolve format issue
    // ui.label(&format!("Api Info: {:?}", api_version));
    // ui.label(api_version);

    // (*ui).new_row();
    // ui.label(&format!("Memsos version: ", env!("CARGO_PKG_VERSION")));

    let mut test_result = true;

    for region in regions.iter() {
        test_result = test_memory(ui, region, *mem_offset);
    }

    if !test_result {
        panic!("Memory test failed");
    }
    ui.show_alert("Test passed!", "");
    loop {}
}

#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    let ui = get_ui!();
    ui.show_alert_unrecoverable(
        "Hubo un error irrecuperable",
        panic.message().as_str().unwrap(),
    )
}
