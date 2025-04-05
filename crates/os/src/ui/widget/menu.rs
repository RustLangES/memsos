use super::{ask::Ask, line::line, text::Text, Widget};
use crate::ui::layout::Layout;
use crate::ui::layout::{vertical::VerticalLayout, LayoutParams};
use crate::ui::writer::{get_ui, height, width};
use crate::{render, PADDING};

pub struct Menu<'a> {
    pub texts: &'a [Text],
    pub ask: Option<Ask<'a>>,
    pixels: isize,
}

impl<'a> Menu<'a> {
    pub fn new(texts: &'a [Text], ask: Option<Ask<'a>>, distance: isize) -> Self {
        Menu {
            texts,
            ask,
            pixels: distance,
        }
    }
    pub fn clear_zone(&self) {
        let mut writer = get_ui();
        let s = self.pixels;
        let w = width() as isize;
        let h = height() as isize;

        writer.clear_zone(
            (
                (PADDING + s).try_into().unwrap(),
                (PADDING + s).try_into().unwrap(),
            ),
            (
                (w - PADDING - s).try_into().unwrap(),
                ((h - PADDING) - s).try_into().unwrap(),
            ),
        );
    }
}

impl Widget for Menu<'_> {
    fn erase(&self, writer: &mut crate::ui::writer::UiWriter) {
        unimplemented!();
    }
    fn render_as_child(
        &self,
        writer: &mut crate::ui::writer::UiWriter,
        args: crate::ui::layout::ChildArgs,
    ) {
        unimplemented!();
    }
    fn render(&self, writer: &mut crate::ui::writer::UiWriter) {
        let w = width() as isize;
        let h = height() as isize;
        let s = self.pixels;
        let layout = VerticalLayout::new(LayoutParams {
            padding: 0,
            start_pos: (
                PADDING as usize + 5 + s as usize,
                PADDING as usize + 5 + s as usize,
            ),
            line_size: None,
            max_y: None,
        });

        render!(
            &line((PADDING + s, PADDING + s), (PADDING - s, (h - PADDING) - s)), // left
            &line(
                (PADDING + s, h - PADDING - s),
                (w - PADDING - s, (h - PADDING) - s)
            ), // down
            &line(
                (w - PADDING - s, PADDING + s),
                (w - PADDING - s, (h - PADDING) - s)
            ), // right
            &line((PADDING + s, PADDING + s), (w - PADDING - s, PADDING + s))    // up
        );

        self.texts.iter().for_each(|t| layout.spawn(t));
    }
    fn spacing(&self) -> usize {
        unimplemented!();
    }
}
