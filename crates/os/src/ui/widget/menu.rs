use super::{ask::Ask, line::line, text::Text, Widget};
use crate::ui::layout::{vertical::VerticalLayout, LayoutParams};
use crate::ui::writer::{height, width};
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
        render!(
            &line((PADDING, PADDING), (PADDING, h - PADDING)),
            &line((PADDING, h - PADDING), (w - PADDING, h - PADDING)),
            &line((w - PADDING, PADDING), (w - PADDING, h - PADDING)),
            &line((PADDING, PADDING), (w - PADDING, PADDING))
        );

        self.texts.iter().for_each(|t| writer.render(t));
    }
    fn spacing(&self) -> usize {
        unimplemented!();
    }
}
