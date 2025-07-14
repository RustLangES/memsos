use core::{cell::SyncUnsafeCell, intrinsics::write_bytes};

use limine::framebuffer::Framebuffer;
use noto_sans_mono_bitmap::RasterizedChar;

use crate::{
    boot::FRAMEBUFFER_REQUEST,
    writer::font::{
        get_char_raster, BORDER_PADDING, CHAR_RASTER_HEIGHT, CHAR_RASTER_WIDTH, LETTER_SPACING, LINE_SPACING
    },
};

pub static WRITER: SyncUnsafeCell<Option<TextWriter>> = SyncUnsafeCell::new(None);

pub struct TextWriter {
    x: usize,
    y: usize,
    framebuffer: Framebuffer<'static>,
}

impl TextWriter {
    pub fn new(framebuffer: Framebuffer<'static>) -> Self {
        Self {
            x: 0,
            y: 0,
            framebuffer,
        }
    }
    pub fn clear(&mut self) {
        let width = self.framebuffer.width();
        let height = self.framebuffer.height();

        for y in 0..height {
            for x in 0..width {
                let pixel_offset = y * self.framebuffer.pitch() + x * 4;
                let offset = usize::try_from(pixel_offset)
                    .expect("Cannot convert the pixel offset to usize");
                unsafe {
                    let buffer = self.framebuffer.addr().add(offset).cast::<u32>();
                    *buffer = 0x0000_0000;
                }
            }
        }
    }
    pub fn write_pixel(&mut self, x: u64, y: u64, byte: u32) {
        let pixel_offset = y * self.framebuffer.pitch() + x * 4;
        let offset =
            usize::try_from(pixel_offset).expect("Cannot convert the pixel offset to usize");
        unsafe {
            let buffer = self.framebuffer.addr().add(offset).cast::<u32>();

            *buffer = byte;
        }
    }
    pub fn newline(&mut self) {
        self.y += CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        self.carriage_return();
    }
    pub fn carriage_return(&mut self) {
        self.x = BORDER_PADDING;
    }
    pub fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            c => {
                let new_xpos = self.x + CHAR_RASTER_WIDTH;
                if new_xpos >= self.framebuffer.width() as usize {
                    self.newline();
                }
                let new_ypos = self.y + CHAR_RASTER_HEIGHT.val() + BORDER_PADDING;

                if new_ypos >= self.framebuffer.height() as usize {
                    self.clear();
                }

                self.write_rendered_char(&get_char_raster(c));
            }
        }
    }
    pub fn write_str(&mut self, s: &str) {
        for char in s.chars() {
            self.write_char(char);
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
}

#[inline]
pub fn init_writer() {
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            let writer = TextWriter::new(framebuffer);

            unsafe {
                *WRITER.get() = Some(writer);
            }
        }
    }
}

#[inline]
pub const fn get_ui() -> &'static mut TextWriter {
    unsafe { WRITER.get().as_mut().expect("WRITER is empty").as_mut().unwrap() }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        let mut buffer = heapless::String::<1024>::new();
        buffer.clear();
        write!(buffer, "{}", format_args!($($arg)*)).expect("Cannot format args");

        $crate::writer::get_ui().write_str(buffer.as_str());
    }
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n");
    };
    ($($arg:tt)*) => {
        let mut buffer = heapless::String::<1024>::new();
        buffer.clear();
        write!(buffer, "{}", format_args!($($arg)*)).expect("Cannot format args");

        $crate::writer::get_ui().write_str(buffer.as_str());
        $crate::writer::get_ui().newline();
    }
}

mod font {
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
}
