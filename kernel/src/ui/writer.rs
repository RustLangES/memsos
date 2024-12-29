use crate::ui::widget::Widget;
use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use core::{ptr, cell::SyncUnsafeCell};

pub static UI_WRITER: SyncUnsafeCell<Option<UiWriter>> = SyncUnsafeCell::new(None);

pub struct UiWriter {
    pub buffer: &'static mut [u8],
    pub info: FrameBufferInfo,
}

impl UiWriter {
    pub fn new(buffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        Self { buffer, info }
    }
    pub fn width(&self) -> usize {
        self.info.width
    }
    pub fn height(&self) -> usize {
        self.info.height
    }
    pub fn clear(&mut self) {
        self.buffer.fill(0);
    }
    pub fn write_pixel(&mut self, x: usize, y: usize, intensity: u8) {
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
        let _ = unsafe { ptr::read_volatile(&self.buffer[byte_offset]) };
    }
    pub fn render<T: Widget>(&mut self, widget: T) {
        widget.render(self);
    }
}

unsafe impl Send for UiWriter {}
unsafe impl Sync for UiWriter {}

#[inline]
pub fn init_ui(buffer: &'static mut [u8], info: FrameBufferInfo) {
   let ui = UiWriter {
       buffer,
       info,
   };

   unsafe {
      *UI_WRITER.get() = Some(ui);
   }
}

#[macro_export]
macro_rules! get_ui {
    () => {{
        unsafe { $crate::ui::writer::UI_WRITER.get().clone().read().expect("UI_WRITER is empty") }
    }};
}

#[macro_export]
macro_rules! render {
   ($widget: expr) => {
      let mut ui = get_ui!();
      ui.render($widget);
    }
}

#[macro_export]
macro_rules! clear {
    () => {
        let mut ui = get_ui!();

        ui.clear();
    };
}

