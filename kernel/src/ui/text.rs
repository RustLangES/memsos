use crate::ui::{widget::Widget, writer::UiWriter};
use core::fmt::{Arguments, Write};
use heapless::String;
use noto_sans_mono_bitmap::{
    get_raster, get_raster_width, FontWeight, RasterHeight, RasterizedChar,
};


const CHAR_RASTER_HEIGHT: RasterHeight = RasterHeight::Size16;
const CHAR_RASTER_WIDTH: usize = get_raster_width(FontWeight::Regular, CHAR_RASTER_HEIGHT);
const BACKUP_CHAR: char = '�';
const FONT_WEIGHT: FontWeight = FontWeight::Regular;
const LINE_SPACING: usize = 2;
const BORDER_PADDING: usize = 1;

const STRING_SIZE: usize = 256;

pub struct Text {
    pub text: String<STRING_SIZE>,
    pub pos: (usize, usize),
}

impl Widget for Text {
    fn render(&self, writer: &mut UiWriter) {
        let mut pos = self.pos;
        for c in self.text.chars() {
            pos = self.write_char(c, writer, pos);
        }
    }
}

pub fn args<'a>(arg: Arguments<'a>, buf: &'a mut String<STRING_SIZE>) -> &'a str {
    buf.clear();
    buf.write_fmt(arg).unwrap();
    buf.as_str()
}


#[macro_export]
macro_rules! text {
  ($pos: expr, $($arg:tt)*) => {{
     let mut buf = heapless::String::<256>::new();
      buf.clear();
      buf.write_fmt(format_args!($($arg)*)).unwrap();
      Text {
         pos: $pos,
         text: buf,
      }
  }}
}


impl Text {
    fn newline(&self, pos: (usize, usize)) -> (usize, usize) {
        let y = pos.1 + CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        (BORDER_PADDING, y)
    }

    fn write_char(&self, c: char, writer: &mut UiWriter, p: (usize, usize)) -> (usize, usize) {
        let mut pos = match c {
            '\n' => self.newline(p),
            _ => p,
        };

        let new_xpos = pos.0 + CHAR_RASTER_WIDTH;
        let new_ypos = pos.1 + CHAR_RASTER_HEIGHT.val() + BORDER_PADDING;
        if new_xpos >= writer.width() {
            pos = self.newline(pos);
        }
        if new_ypos >= writer.height() {
            writer.clear();
        }

        for (y, row) in get_char_raster(c).raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                writer.write_pixel(pos.0 + x, pos.1 + y, *byte);
            }
        }
        (pos.0 + CHAR_RASTER_WIDTH, pos.1)
    }
}



fn get_char_raster(c: char) -> RasterizedChar {
    fn get(c: char) -> Option<RasterizedChar> {
        get_raster(c, FONT_WEIGHT, CHAR_RASTER_HEIGHT)
    }
    get(c).unwrap_or_else(|| get(BACKUP_CHAR).expect("Should get raster of backup char."))
}
