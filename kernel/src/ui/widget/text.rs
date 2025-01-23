use crate::ui::{layout::LayoutChild, widget::Widget, writer::UiWriter};
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

#[derive(Clone, Debug)]
pub struct Text {
    pub text: String<STRING_SIZE>,
    pub pos: (usize, usize),
}

impl Widget for Text {
    fn render(&self, writer: &mut UiWriter) {
        self.text
            .chars()
            .fold(self.pos, |acc, c| self.write_char(c, writer, acc, writer.width()));    
        }
    fn erase(&self, writer: &mut UiWriter) {
        self.text
            .chars()
            .fold(self.pos, |acc, _c| self.write_char(' ', writer, acc, writer.width()));
    }
}

impl LayoutChild for Text {
    fn render_child(
        &self,
        writer: &mut UiWriter,
        args: crate::ui::layout::LayoutArgs,
    ) {
        self.text
            .chars()
            .fold(args.pos, |acc, c| self.write_char(c, writer, acc, args.line_size));
    }
    fn spacing(&self) -> usize {
        self.pos.1 + CHAR_RASTER_HEIGHT.val() + LINE_SPACING
    }
}

#[macro_export]
macro_rules! text {
  ($pos: expr, $($arg:tt)*) => {{
      $crate::ui::widget::text::Text::new_from_args(format_args!($($arg)*), $pos)
  }};
  ($($arg:tt)*) => {{
      $crate::ui::widget::text::Text::new_from_args(format_args!($($arg)*), (0, 0))
  }}
}

impl Text {
    pub fn new(text: String<STRING_SIZE>, pos: (usize, usize)) -> Self {
        Self { text, pos }
    }
    pub fn new_from_args<'a>(args: Arguments<'a>, pos: (usize, usize)) -> Self {
        let mut buffer = String::<STRING_SIZE>::new();
        buffer.clear();
        write!(&mut buffer, "{args}").expect("Could not format args");

        Self { text: buffer, pos }
    }
    fn newline(&self, pos: (usize, usize)) -> (usize, usize) {
        let y = pos.1 + CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        (23, y)
    }

    fn write_char(&self, c: char, writer: &mut UiWriter, p: (usize, usize), line_size: usize) -> (usize, usize) {
        let mut pos = match c {
            '\n' => self.newline(p),
            _ => p,
        };

        let new_xpos = pos.0 + CHAR_RASTER_WIDTH;
        let new_ypos = pos.1 + CHAR_RASTER_HEIGHT.val() + BORDER_PADDING;
        if new_ypos >= writer.height() {
           writer.clear(); 
        }
        if new_xpos >= line_size {
            pos = self.newline(pos);
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
