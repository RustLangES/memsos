use crate::ui::layout::{vertical::VerticalLayout, Layout, LayoutChild, LayoutParams};
use crate::ui::widget::text::TextStyle;
use crate::ui::widget::Widget;
use crate::{render, styled_text, text};

pub struct Ask {
    pub options: &'static [&'static str],
}

impl Ask {
    pub fn new(opts: &'static [&'static str]) -> Self {
        Self { options: opts }
    }
}

impl Widget for Ask {
    fn render(&self, _writer: &mut crate::ui::writer::UiWriter) {
        let current = 0;
        let layout = VerticalLayout::new(LayoutParams {
            max_y: None,
            padding: 0,
            start_pos: (0, 0),
            line_size: None,
        });
        for i in 0..self.options.len() {
            let w = &self.options[i];
            let t = {
                if current == i {
                    styled_text!(layout.gen_pos(), TextStyle { invert: true }, "{w}")
                } else {
                    text!(layout.gen_pos(), "{w}")
                }
            };
            layout.margin(t.spacing());
            render!(&t);
        }
    }
    fn erase(&self, _writer: &mut crate::ui::writer::UiWriter) {
        unimplemented!();
    }
}
