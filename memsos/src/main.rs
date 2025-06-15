#![feature(uefi_std)]

slint::include_modules!();

mod platform;

use crate::platform::Platform;
use arch::mem::{get_usable_mem, write};
use arch::protocols::UefiProtocols;
use memtest::own_addr::own_addr_check;
use slint::{ComponentHandle, Timer, ToSharedString, Weak};
use uefi::mem::memory_map::MemoryMap as MemoryMapTrait;
use std::os::uefi as uefi_std;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicI32, AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::Duration;
use uefi::boot::{
    create_event, get_handle_for_protocol, memory_map, open_protocol_exclusive, set_timer, EventNotifyFn, EventType, MemoryType, Tpl
};
use uefi::proto::console::gop::GraphicsOutput;
use uefi::runtime::ResetType;
use uefi::{boot, Event, Handle, Status};

fn setup() {
    let st = uefi_std::env::system_table();
    let ih = uefi_std::env::image_handle();

    unsafe {
        uefi::table::set_system_table(st.as_ptr().cast());

        let ih = Handle::from_ptr(ih.as_ptr().cast()).unwrap();
        uefi::boot::set_image_handle(ih);
    }
}

static COUNT: AtomicI32 = AtomicI32::new(0);

fn main() {
    setup();
    slint::platform::set_platform(Box::<Platform>::default()).unwrap();

    let ui = Ui::new().unwrap();
    let weak = ui.as_weak();
    let timer = Timer::default();
   
    /*
    timer.start(
        slint::TimerMode::Repeated,
        Duration::from_millis(10),
        move || {
            weak.upgrade()
                .unwrap()
                .set_total_cpu(COUNT.load(Ordering::SeqCst));
            COUNT.fetch_add(1, Ordering::SeqCst);
        },
    );
    */

    ui.run().unwrap();

    uefi::runtime::reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
}
