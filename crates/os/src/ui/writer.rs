use crate::ui::widget::Widget;
use crate::PADDING;
use limine::framebuffer::Framebuffer;
use core::{cell::SyncUnsafeCell, ptr};
use crate::request::FRAMEBUFFER_REQUEST;

pub static UI_WRITER: SyncUnsafeCell<Option<UiWriter>> = SyncUnsafeCell::new(None);

pub struct UiWriter {
    pub buffer: Framebuffer<'static>,
}

impl UiWriter {
    pub const fn new(buffer: Framebuffer<'static>) -> Self {
        Self { buffer }
    }
    pub fn width(&self) -> usize {
        usize::try_from(self.buffer.width()).expect("Cannot convert u64 to usize")
    }
    pub fn height(&self) -> usize {
        usize::try_from(self.buffer.height()).expect("Cannot convert u64 to usize")
    }
    pub fn clear_zone(&mut self, from: (u64, u64), to: (u64, u64)) {
        let padding:  u64 = PADDING.try_into().unwrap();
        for x in from.0..=to.0 {
            for y in from.1..=to.1 {
                if x < padding
                    || x >= self.width() as u64 - padding
                    || y < padding
                    || y >= self.height() as u64 - padding
                {
                    continue;
                }

                self.write_pixel(x, y, 0);
            }
        }
    }
    pub fn clear(&mut self) {
        let width = self.width() as u64;
        let height = self.height() as u64;

        for y in 0..height {
            for x in 0..width {
                let pixel_offset = y * self.buffer.pitch() + x * 4;
                let offset = usize::try_from(pixel_offset)
                    .expect("Cannot convert the pixel offset to usize");
                unsafe {
                    let buffer = self.buffer.addr().add(offset).cast::<u32>();
                    *buffer = 0x0000_0000;
                }
            }
        }
    }
    pub fn write_pixel(&mut self, x: u64, y: u64, color: u32) {
        let pixel_offset = y * self.buffer.pitch() + x * 4;
        let offset =
            usize::try_from(pixel_offset).expect("Cannot convert the pixel offset to usize");
        unsafe {
            let buffer = self.buffer.addr().add(offset).cast::<u32>();

            *buffer = color;
        }
    }
    pub fn render<T: Widget>(&mut self, widget: &T) {
        widget.render(self);
    }
    pub fn erase<T: Widget>(&mut self, widget: &T) {
        widget.erase(self);
    }
}

unsafe impl Send for UiWriter {}
unsafe impl Sync for UiWriter {}

#[inline]
pub fn init_ui() {
     if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            let writer = UiWriter::new(framebuffer);
            
            unsafe { *UI_WRITER.get() = Some(writer); }
        }
    }
}

#[inline]
pub const fn get_ui() -> UiWriter {
    unsafe { UI_WRITER.get().read().expect("UI_WRITER is empty") }
}

#[macro_export]
macro_rules! render {
    ($widget: expr) => {
        let mut ui = $crate::ui::writer::get_ui();
        ui.render($widget);
    };
    ( $( $widget:expr ),* ) => {
        let mut ui = $crate::ui::writer::get_ui();
        $(
            ui.render($widget);
        )*
    };
}

#[macro_export]
macro_rules! layout {
    ( $layout: expr, $( $widget:expr ),* )  => {
        $(
            $layout.spawn($widget);
        )*
    };
}

#[macro_export]
macro_rules! erase {
    ($widget: expr) => {
        let mut ui = $crate::ui::writer::get_ui();
        ui.erase($widget);
    };
    ( $( $widget:expr ),* ) => {
        let mut ui = $crate::ui::writer::get_ui();
        $(
            ui.erase($widget);
        )*
    };
}

#[inline]
pub fn clear() {
    let mut ui = get_ui();

    ui.clear();
}

#[inline]
pub fn clear_zone(from: (usize, usize), to: (usize, usize)) {
    let mut ui = get_ui();

    ui.clear_zone(from, to);
}
