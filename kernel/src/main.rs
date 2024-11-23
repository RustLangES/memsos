#![no_std]
#![no_main]
#![allow(clippy::similar_names)]
#![feature(sync_unsafe_cell)]

mod asm;
mod drivers;
mod test;
mod writer;

use bootloader_api::{config::Mapping, entry_point, BootInfo, BootloaderConfig};
use drivers::keyboard::{Key, Scanner};
use heapless::String;

use test::{read_from_memory, test_memory};
use writer::{FrameBufferWriter, WRITER};

use core::{fmt::Write, panic::PanicInfo};

const CONFIG: BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    //config.kernel_stack_size = 100 * 1024;
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};
entry_point!(kernel_main, config = &CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let memory = &boot_info.memory_regions;
    let regions = memory.iter();
    let api_version = &boot_info.api_version;
    let framebuffer = boot_info.framebuffer.take().unwrap();
    let info = framebuffer.info();
    let buffer = framebuffer.into_buffer();

    let memsos_version = env!("CARGO_PKG_VERSION");

    init_writer!(buffer, info);
    clean!();

    println!("Api Info: {:?}", api_version);
    println!("Memsos version: {}", memsos_version);

    for region in regions {
        test_memory(region);
    }

    loop {}
}

#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    clean!();
    println!("{}", panic.message());
    loop {}
}
