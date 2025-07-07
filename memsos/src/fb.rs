use core::ptr::write_bytes;

use alloc::string::String;
use noto_sans_mono_bitmap::{
    FontWeight, RasterHeight, RasterizedChar, get_raster, get_raster_width,
};
use r_efi::protocols::graphics_output::{
    ModeInformation, PIXEL_BLUE_GREEN_RED_RESERVED_8_BIT_PER_COLOR,
    PIXEL_RED_GREEN_BLUE_RESERVED_8_BIT_PER_COLOR,
};

const LETTER_SPACING: usize = 0;
const LINE_SPACING: usize = 2;
pub const CHAR_RASTER_HEIGHT: RasterHeight = RasterHeight::Size16;
const BORDER_PADDING: usize = 1;
pub const FONT_WEIGHT: FontWeight = FontWeight::Regular;

pub const CHAR_RASTER_WIDTH: usize = get_raster_width(FontWeight::Regular, CHAR_RASTER_HEIGHT);
fn get_char_raster(c: char) -> RasterizedChar {
    fn get(c: char) -> Option<RasterizedChar> {
        get_raster(c, FONT_WEIGHT, CHAR_RASTER_HEIGHT)
    }
    get(c).unwrap_or_else(|| get('?').expect("Should get raster of backup char."))
}

pub struct Framebuffer<'a> {
    pub version: u32,
    pub fb: &'a mut [u8],
    pub info: ModeInformation,
}

impl<'a> Framebuffer<'a> {
    pub fn clear(&mut self) {
        self.fb.fill(0x00);
    }
}

pub struct TextRender {
    pub fb: Framebuffer<'static>,
    pub x: usize,
    pub y: usize,
    pub stride: usize,
}

impl TextRender {
    pub fn new(framebuffer: Framebuffer<'static>) -> Self {
        let stride = 4 * framebuffer.info.pixels_per_scan_line;

        Self {
            fb: framebuffer,
            x: 0,
            y: 0,
            stride: stride as usize,
        }
    }
    pub fn newline(&mut self) {
        self.y += CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        self.carriage_return();
    }
    pub fn carriage_return(&mut self) {
        self.x = BORDER_PADDING;
    }
    pub fn clear(&mut self) {
        self.x = BORDER_PADDING;
        self.y = BORDER_PADDING;

        self.fb.clear();
    }
    const fn width(&self) -> usize {
        self.fb.info.horizontal_resolution as usize
    }
    const fn height(&self) -> usize {
        self.fb.info.vertical_resolution as usize
    }
    pub fn write_str(&mut self, str: &str) {
        for i in str.chars() {
            self.write_char(i);
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
                self.write_renderer_char(get_char_raster(c));
            }
        }
    }
    fn write_renderer_char(&mut self, rendered_char: RasterizedChar) {
        for (y, row) in rendered_char.raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                self.write_pixel(self.x + x, self.y + y, *byte);
            }
        }
        self.x += rendered_char.width() + LETTER_SPACING;
    }
    fn write_pixel(&mut self, x: usize, y: usize, byte: u8) {
        let pixel_offset = self.stride * y + 4 * x;
        let color = match self.fb.info.pixel_format {
            PIXEL_RED_GREEN_BLUE_RESERVED_8_BIT_PER_COLOR => [byte, byte, byte / 2, 0],
            PIXEL_BLUE_GREEN_RED_RESERVED_8_BIT_PER_COLOR => [byte / 2, byte, byte, 0],
            _ => {
                panic!("unknown");
            }
        };
        let bytes_per_pixel = 4;
        let byte_offset = pixel_offset * bytes_per_pixel;
        self.fb.fb[byte_offset..(byte_offset + bytes_per_pixel)]
            .copy_from_slice(&color[..bytes_per_pixel]);
    }
}
