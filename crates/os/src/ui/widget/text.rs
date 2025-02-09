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

#[derive(Debug, Clone, Default)]
pub struct TextStyle {
    pub invert: bool,
}

impl TextStyle {
    pub fn apply(&self, byte: u8) -> u8 {
        let mut final_byte = byte;

        if self.invert {
            final_byte = byte ^ 0xFF;
        }

        final_byte
    }
}

#[derive(Clone, Debug)]
pub struct Text {
    pub text: String<STRING_SIZE>,
    pub pos: (usize, usize),
    pub style: TextStyle,
}

impl Widget for Text {
    fn render(&self, writer: &mut UiWriter) {
        self.text.chars().fold(self.pos, |acc, c| {
            self.write_char(c, writer, acc, writer.width())
        });
    }
    fn erase(&self, writer: &mut UiWriter) {
        self.text.chars().fold(self.pos, |acc, _c| {
            self.write_char(' ', writer, acc, writer.width())
        });
    }
}

impl LayoutChild for Text {
    fn render_child(&self, writer: &mut UiWriter, args: crate::ui::layout::LayoutArgs) {
        self.text.chars().fold(args.pos, |acc, c| {
            self.write_char(c, writer, acc, args.line_size)
        });
    }
    fn spacing(&self) -> usize {
        self.pos.1 + CHAR_RASTER_HEIGHT.val() + LINE_SPACING
    }
}

#[macro_export]
macro_rules! text {
  ($pos: expr, $($arg:tt)*) => {{
      $crate::ui::widget::text::Text::new_from_args(format_args!($($arg)*), $pos, $crate::ui::widget::text::TextStyle { invert: false })
  }};
  ($($arg:tt)*) => {{
      $crate::ui::widget::text::Text::new_from_args(format_args!($($arg)*), (0, 0), $crate::ui::widget::text::TextStyle { invert: false })
  }}
}

#[macro_export]
macro_rules! styled_text {
  ($pos: expr, $style: expr, $($arg:tt)*) => {{
      $crate::ui::widget::text::Text::new_from_args(format_args!($($arg)*), $pos, $style)
  }};
}

impl Text {
    pub const fn new(text: String<STRING_SIZE>, pos: (usize, usize), style: TextStyle) -> Self {
        Self { text, pos, style }
    }
    pub fn new_from_args(args: Arguments<'_>, pos: (usize, usize), style: TextStyle) -> Self {
        let mut buffer = String::<STRING_SIZE>::new();
        buffer.clear();
        write!(&mut buffer, "{args}").expect("Could not format args");

        Self {
            text: buffer,
            pos,
            style,
        }
    }
    fn newline(pos: (usize, usize)) -> (usize, usize) {
        let y = pos.1 + CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        (23, y)
    }

    fn write_char(
        &self,
        c: char,
        writer: &mut UiWriter,
        p: (usize, usize),
        line_size: usize,
    ) -> (usize, usize) {
        let mut pos = match c {
            '\n' => Text::newline(p),
            _ => p,
        };

        if c == '\n' {
            return pos;
        }

        let new_xpos = pos.0 + CHAR_RASTER_WIDTH;
        let new_ypos = pos.1 + CHAR_RASTER_HEIGHT.val() + BORDER_PADDING;
        if new_ypos >= writer.height() {
            writer.clear();
        }
        if new_xpos >= line_size {
            pos = Text::newline(pos);
        }

        for (y, row) in get_char_raster(c).raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                let pixel = self.style.apply(*byte);
                writer.write_pixel(pos.0 + x, pos.1 + y, pixel);
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
