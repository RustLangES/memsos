use crate::ui::widget::Widget;
use crate::PADDING;
use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use core::{cell::SyncUnsafeCell, ptr};

pub static UI_WRITER: SyncUnsafeCell<Option<UiWriter>> = SyncUnsafeCell::new(None);

pub struct UiWriter {
    pub buffer: &'static mut [u8],
    pub info: FrameBufferInfo,
}

impl UiWriter {
    pub const fn new(buffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        Self { buffer, info }
    }
    pub const fn width(&self) -> usize {
        self.info.width
    }
    pub const fn height(&self) -> usize {
        self.info.height
    }
    pub fn clear_zone(&mut self, from: (usize, usize), to: (usize, usize)) {
        let padding: usize = PADDING.try_into().unwrap();
        for x in from.0..=to.0 {
            for y in from.1..=to.1 {
                if x < padding
                    || x >= self.width() - padding
                    || y < padding
                    || y >= self.height() - padding
                {
                    continue;
                }

                self.write_pixel(x, y, 0);
            }
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            ptr::write_bytes(self.buffer.as_mut_ptr(), 0, self.buffer.len());
        }
    }
    pub fn write_pixel(&mut self, x: usize, y: usize, intensity: u8) {
        if x >= self.info.width || y >= self.info.height {
            return;
        }

        let pixel_offset = y * self.info.stride + x;
        let color = match self.info.pixel_format {
            PixelFormat::Rgb => [intensity, intensity, intensity / 2, 0],
            PixelFormat::Bgr => [intensity / 2, intensity, intensity, 0],
            PixelFormat::U8 => [if intensity > 200 { 0xf } else { 0 }, 0, 0, 0],
            other => {
                panic!("pixel format {:?} not supported in logger", other)
            }
        };
        let bytes_per_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;

        self.buffer[byte_offset..(byte_offset + bytes_per_pixel)]
            .copy_from_slice(&color[..bytes_per_pixel]);
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
pub fn init_ui(buffer: &'static mut [u8], info: FrameBufferInfo) {
    let ui = UiWriter { buffer, info };

    unsafe {
        *UI_WRITER.get() = Some(ui);
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
