use crate::requests::FRAMEBUFFER_REQUEST;
use alloc::{boxed::Box, string::String};
use core::fmt;
use lazy_static::lazy_static;
use limine::framebuffer::Framebuffer;
use spin::Mutex;

use noto_sans_mono_bitmap::{
    FontWeight, RasterHeight, RasterizedChar, get_raster, get_raster_width,
};

pub const LINE_SPACING: usize = 2;
pub const LETTER_SPACING: usize = 0;
pub const BORDER_PADDING: usize = 1;

pub const CHAR_RASTER_HEIGHT: RasterHeight = RasterHeight::Size16;
pub const BACKUP_CHAR: char = '�';
pub const FONT_WEIGHT: FontWeight = FontWeight::Regular;
pub const CHAR_RASTER_WIDTH: usize = get_raster_width(FONT_WEIGHT, CHAR_RASTER_HEIGHT);

pub fn get_char_raster(c: char) -> RasterizedChar {
    fn get(c: char) -> Option<RasterizedChar> {
        get_raster(c, FONT_WEIGHT, CHAR_RASTER_HEIGHT)
    }
    get(c).unwrap_or_else(|| get(BACKUP_CHAR).expect("Should get raster of backup char."))
}

lazy_static! {
    pub static ref WRITER: Mutex<Option<FrameBufferWriter<'static>>> = Mutex::new(None);
}

pub fn init_writer() {
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            let writer = FrameBufferWriter::new(Box::new(framebuffer));
            let mut writer_lock = WRITER.lock();
            *writer_lock = Some(writer);
        }
    }
}

pub struct FrameBufferWriter<'a> {
    buffer: Box<Framebuffer<'a>>,
    x: usize,
    y: usize,
}

impl<'a> FrameBufferWriter<'a> {
    pub fn new(buffer: Box<Framebuffer<'a>>) -> Self {
        Self { buffer, x: 0, y: 0 }
    }
    pub fn newline(&mut self) {
        self.y += CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        self.x = 0;
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
    pub fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            c => {
                let new_xpos = self.x + CHAR_RASTER_WIDTH;
                if new_xpos >= self.width() {
                    self.newline();
                }
                let new_ypos = self.y + CHAR_RASTER_HEIGHT.val() + BORDER_PADDING;
                if new_ypos >= self.height() {
                    self.clear();
                }
                self.write_rendered_char(&get_char_raster(c));
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
    fn write_rendered_char(&mut self, rendered_char: &RasterizedChar) {
        for (y, row) in rendered_char.raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                let pixel_x = (self.x + x) as u64;
                let pixel_y = (self.y + y) as u64;
                let intensity = u32::from(*byte);
                let color = (intensity << 16) | (intensity << 8) | intensity;

                self.write_pixel(pixel_x, pixel_y, color);
            }
        }
        self.x += rendered_char.width() + LETTER_SPACING;
    }
    fn width(&self) -> usize {
        usize::try_from(self.buffer.width()).expect("Cannot convert u64 to usize")
    }
    fn height(&self) -> usize {
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
    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        if let Some(s) = &args.as_str() {
            self.write_str(s).expect("Could not write in the screen");
        } else {
            let mut buffer = String::new();
            write!(&mut buffer, "{args}").expect("Could not format args");
            self.write_str(&buffer)
                .expect("Could not write in the screen");
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        let mut lock = $crate::writer::WRITER.lock();
        let writer = lock.as_mut().expect("Writer is null");
        writer.write_fmt(format_args!($($arg)*)).expect("Could not write the message");
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
