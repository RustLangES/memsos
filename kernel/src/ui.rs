mod base;

use core::cell::SyncUnsafeCell;

pub use base::*;

pub static UI: SyncUnsafeCell<Option<MemsosUI>> = SyncUnsafeCell::new(None);

#[macro_export]
macro_rules! init_ui {
    ($buffer: expr, $info: expr) => {
        unsafe {
            let buff = core::mem::transmute::<
                &'_ mut $crate::ui::MemsosUIWriter<$crate::ui::Color>,
                &'static mut $crate::ui::MemsosUIWriter<$crate::ui::Color>,
            >(&mut MemsosUIWriter::new(
                $info.width.try_into().unwrap(),
                $info.height.try_into().unwrap(),
                $info.stride.try_into().unwrap(),
                $info.bytes_per_pixel.try_into().unwrap(),
                $info.pixel_format,
                $buffer,
            ));
            (*UI.get()).get_or_insert_with(|| MemsosUI::new(buff))
        }
    };
}

#[macro_export]
macro_rules! get_ui {
    () => {{
        let ui = unsafe { (*$crate::ui::UI.get()).as_mut().unwrap() };
        ui.clear();
        ui
    }};
}
