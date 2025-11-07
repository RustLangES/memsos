// Note, at present the ui will remain as simple as this, at some point it may become a little more complex.
#![no_std]
#![feature(sync_unsafe_cell)]

pub mod display;

use boot::requests::FRAMEBUFFER_REQUEST;
use core::{cell::SyncUnsafeCell, fmt};
use embedded_graphics::{pixelcolor::Rgb888, prelude::RgbColor};
use limine::framebuffer::Framebuffer;
use noto_sans_mono_bitmap::{
    FontWeight, RasterHeight, RasterizedChar, get_raster, get_raster_width,
};

use crate::display::FbDisplay;

pub const LINE_SPACING: usize = 2;
pub const LETTER_SPACING: usize = 0;
pub const BORDER_PADDING: usize = 1;

pub const CHAR_RASTER_HEIGHT: RasterHeight = RasterHeight::Size16;
pub const BACKUP_CHAR: char = '?';
pub const FONT_WEIGHT: FontWeight = FontWeight::Regular;
pub const CHAR_RASTER_WIDTH: usize = get_raster_width(FONT_WEIGHT, CHAR_RASTER_HEIGHT);

/// # Panics
///
/// It panics if the `backup_char` is invalid
#[must_use]
pub fn get_char_raster(c: char) -> RasterizedChar {
    fn get(c: char) -> Option<RasterizedChar> {
        get_raster(c, FONT_WEIGHT, CHAR_RASTER_HEIGHT)
    }
    get(c).unwrap_or_else(|| get(BACKUP_CHAR).expect("Should get raster of backup char."))
}

pub static WRITER: SyncUnsafeCell<Option<FrameBufferWriter>> = SyncUnsafeCell::new(None);
pub static UI_WRITER: SyncUnsafeCell<Option<FbDisplay>> = SyncUnsafeCell::new(None);

pub fn init_writer() {
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response()
        && let Some(framebuffer) = framebuffer_response.framebuffers().next()
    {
        let writer = FrameBufferWriter::new(framebuffer);

        unsafe { *WRITER.get() = Some(writer) }
    }
}

pub fn init_ui() {
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response()
        && let Some(framebuffer) = framebuffer_response.framebuffers().next()
    {
        let writer = FbDisplay::new(framebuffer);

        unsafe { *UI_WRITER.get() = Some(writer) }
    }
}

pub struct FrameBufferWriter<'a> {
    buffer: Framebuffer<'a>,
    pub color: Rgb888,
    pub x: usize,
    pub y: usize,
}

impl<'a> FrameBufferWriter<'a> {
    #[must_use]
    pub fn new(buffer: Framebuffer<'a>) -> Self {
        Self {
            buffer,
            x: 0,
            y: 0,
            color: Rgb888::WHITE,
        }
    }
    pub fn newline(&mut self) {
        self.y += CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        self.x = 0;
    }
    /// # Panics
    ///
    /// it panics if `pixel_offset` can't be usize
    pub fn clear(&mut self) {
        let width = self.width() as u64;
        let height = self.height() as u64;

        for y in 0..height {
            for x in 0..width {
                let pixel_offset = y * self.buffer.pitch() + x * 4;
                let offset = usize::try_from(pixel_offset)
                    .expect("Cannot convert the pixel offset to usize");
                unsafe {
                    #[allow(clippy::cast_ptr_alignment)]
                    let buffer = self.buffer.addr().add(offset).cast::<u32>();
                    *buffer = 0x0000_0000;
                }
            }
        }
    }
    pub fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            c => {
                let new_horizontal_pos = self.x + CHAR_RASTER_WIDTH;
                if new_horizontal_pos >= self.width() {
                    self.newline();
                }
                let new_vertical_pos = self.y + CHAR_RASTER_HEIGHT.val() + BORDER_PADDING;
                if new_vertical_pos >= self.height() {
                    self.clear();
                }
                self.write_rendered_char(&get_char_raster(c));
            }
        }
    }
    /// # Panics
    ///
    /// it panics if `pixel_offset` can't be usize
    pub fn write_pixel(&mut self, x: u64, y: u64, color: u32) {
        let pixel_offset = y * self.buffer.pitch() + x * 4;
        let offset =
            usize::try_from(pixel_offset).expect("Cannot convert the pixel offset to usize");
        unsafe {
            #[allow(clippy::cast_ptr_alignment)]
            let buffer = self.buffer.addr().add(offset).cast::<u32>();

            *buffer = color;
        }
    }
    fn write_rendered_char(&mut self, rendered_char: &RasterizedChar) {
        for (y, row) in rendered_char.raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                let pixel_x = (self.x + x) as u64;
                let pixel_y = (self.y + y) as u64;
                let intensity = u32::from(*byte);

                let color = ((intensity & self.color.r() as u32) << 16)
                    | ((intensity & self.color.g() as u32) << 8)
                    | (intensity & self.color.b() as u32);

                self.write_pixel(pixel_x, pixel_y, color);
            }
        }
        self.x += rendered_char.width() + LETTER_SPACING;
    }
    pub fn width(&self) -> usize {
        usize::try_from(self.buffer.width()).expect("Cannot convert u64 to usize")
    }
    pub fn height(&self) -> usize {
        usize::try_from(self.buffer.height()).expect("Cannot convert u64 to usize")
    }
    fn carriage_return(&mut self) {
        self.x = BORDER_PADDING;
    }
}

impl fmt::Write for FrameBufferWriter<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for char in s.chars() {
            self.write_char(char);
        }

        Ok(())
    }
}

/// # Panics
///
///  It may cause panic if this function is called before the writer is initialized.
pub fn get_fb_writer() -> &'static mut FrameBufferWriter<'static> {
    unsafe { WRITER.get().as_mut().unwrap().as_mut().unwrap() }
}

/// # Panics
///
///  It may cause panic if this function is called before the ui writer is initialized.
pub fn get_ui_writer() -> &'static mut FbDisplay {
    unsafe { UI_WRITER.get().as_mut().unwrap().as_mut().unwrap() }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        let writer = $crate::get_fb_writer();

        write!(writer, "{}", format_args!($($arg)*)).expect("Cannot format args");
    }};

}

#[macro_export]
macro_rules! color_print {
    ($color: expr, $($arg:tt)*) => {{
        let writer = $crate::get_fb_writer();
        let c = writer.color;

        writer.color = $color;

        write!(writer, "{}\n", format_args!($($arg)*)).expect("Cannot format args");

        writer.color = c;
    }};
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::print!($($arg)*);
        $crate::print!("\n");
    }};

}
