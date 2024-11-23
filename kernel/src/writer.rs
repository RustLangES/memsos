use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use core::{
    cell::SyncUnsafeCell,
    fmt::{self, Write},
    ptr,
};
use font_constants::BACKUP_CHAR;
use heapless::String;
use noto_sans_mono_bitmap::{
    get_raster, get_raster_width, FontWeight, RasterHeight, RasterizedChar,
};

const LINE_SPACING: usize = 2;
const LETTER_SPACING: usize = 0;
const BORDER_PADDING: usize = 1;
mod font_constants {
    use super::{get_raster_width, FontWeight, RasterHeight};
    pub const CHAR_RASTER_HEIGHT: RasterHeight = RasterHeight::Size16;
    pub const CHAR_RASTER_WIDTH: usize = get_raster_width(FontWeight::Regular, CHAR_RASTER_HEIGHT);
    pub const BACKUP_CHAR: char = '�';

    pub const FONT_WEIGHT: FontWeight = FontWeight::Regular;
}

pub static WRITER: SyncUnsafeCell<Option<FrameBufferWriter>> = SyncUnsafeCell::new(None);

pub struct FrameBufferWriter {
    pub framebuffer: &'static mut [u8],
    pub info: FrameBufferInfo,
    pub x_pos: usize,
    pub y_pos: usize,
}

fn get_char_raster(c: char) -> RasterizedChar {
    fn get(c: char) -> Option<RasterizedChar> {
        get_raster(
            c,
            font_constants::FONT_WEIGHT,
            font_constants::CHAR_RASTER_HEIGHT,
        )
    }
    get(c).unwrap_or_else(|| get(BACKUP_CHAR).expect("Should get raster of backup char."))
}

impl FrameBufferWriter {
    pub fn newline(&mut self) {
        self.y_pos += font_constants::CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        self.carriage_return();
    }

    fn carriage_return(&mut self) {
        self.x_pos = BORDER_PADDING;
    }

    pub fn clear(&mut self) {
        self.x_pos = BORDER_PADDING;
        self.y_pos = BORDER_PADDING;
        self.framebuffer.fill(0);
    }

    fn width(&self) -> usize {
        self.info.width
    }

    fn height(&self) -> usize {
        self.info.height
    }

    fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            c => {
                let new_xpos = self.x_pos + font_constants::CHAR_RASTER_WIDTH;
                if new_xpos >= self.width() {
                    self.newline();
                }
                let new_ypos =
                    self.y_pos + font_constants::CHAR_RASTER_HEIGHT.val() + BORDER_PADDING;
                if new_ypos >= self.height() {
                    self.clear();
                }
                self.write_rendered_char(get_char_raster(c));
            }
        }
    }

    fn write_rendered_char(&mut self, rendered_char: RasterizedChar) {
        for (y, row) in rendered_char.raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                self.write_pixel(self.x_pos + x, self.y_pos + y, *byte);
            }
        }
        self.x_pos += rendered_char.width() + LETTER_SPACING;
    }

    fn write_pixel(&mut self, x: usize, y: usize, intensity: u8) {
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
        self.framebuffer[byte_offset..(byte_offset + bytes_per_pixel)]
            .copy_from_slice(&color[..bytes_per_pixel]);
        let _ = unsafe { ptr::read_volatile(&self.framebuffer[byte_offset]) };
    }
    pub fn clean(&mut self) {
        let framebuffer_ptr = self.framebuffer.as_mut_ptr();
        unsafe { ptr::write_bytes(framebuffer_ptr, 0, self.framebuffer.len()) };
    }
}

unsafe impl Send for FrameBufferWriter {}
unsafe impl Sync for FrameBufferWriter {}

impl fmt::Write for FrameBufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}

pub fn args<'a>(arg: fmt::Arguments<'a>, buf: &'a mut String<256>) -> &'a str {
    buf.clear();
    buf.write_fmt(arg).unwrap();
    buf.as_str()
}

#[macro_export]
macro_rules! init_writer {
    ($framebuffer: expr, $info: expr) => {
        let writer = FrameBufferWriter {
            framebuffer: $framebuffer,
            info: $info,
            x_pos: 0,
            y_pos: 0,
        };

        unsafe {
            *WRITER.get() = Some(writer);
        }
    };
}

#[macro_export]
macro_rules! print { ($($arg:tt)*) => { {
    let mut buf = String::<256>::new();
    let writer = unsafe { (*$crate::writer::WRITER.get()).as_mut().unwrap() };
    writer.write_str($crate::writer::args(format_args!($($arg)*), &mut buf)).unwrap();
    }};
}

#[macro_export]
macro_rules! println {
    () => {
        let writer = unsafe { $crate::writer::WRITER.unwrap() };
        writer.newline();
    };
    ($($arg:tt)*) => {{
        let writer = unsafe { (*$crate::writer::WRITER.get()).as_mut().unwrap() };
        $crate::print!($($arg)*);
        writer.newline();
    }};
}

#[macro_export]
macro_rules! clean {
    () => {
        let writer = unsafe { (*$crate::writer::WRITER.get()).as_mut().unwrap() };

        writer.clean();
    };
}
