#![no_std]
#![no_main]
#![allow(clippy::similar_names)]
#![feature(sync_unsafe_cell)]

mod asm;
mod drivers;
mod mem;
mod memtest;
mod power;
mod writer;

use bootloader_api::{
    config::Mapping, entry_point, info::MemoryRegionKind, BootInfo, BootloaderConfig,
};
use drivers::keyboard::{Key, Keyboard, KeyState};
use heapless::String;
use core::{arch::asm, fmt::Write, panic::PanicInfo};
use memtest::test_memory;
use power::reboot::reboot;
use writer::{FrameBufferWriter, WRITER};

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

    let memsos_version = env!("CARGO_PKG_VERSION");
    init_writer!(buffer, info);
    clean!();

    let Some(mem_offset) = physical else { loop {} };

    println!("Api Info: {:?}", api_version);
    println!("Memsos version: {}", memsos_version);

    let mut test_result = true;

    for region in regions.iter() {
        test_result = test_memory(region, *mem_offset);
    }

    if !test_result {
        panic!("Memory test failed");
    }

    println!("Test passed!");

    loop {}
}

#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    clean!();
    println!("{:?}", panic);
    let keyboard = Keyboard;
    println!("Press space to reboot your computer!");
    keyboard.wait_key(Key::Space);

    reboot();
    loop {}
}
