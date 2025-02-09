use crate::ui::layout::{vertical::VerticalLayout, Layout, LayoutChild, LayoutParams};
use crate::ui::widget::text::{Text, TextStyle};
use crate::ui::widget::Widget;
use crate::{render, styled_text, text};

pub struct Ask<'a> {
    pub options: &'a [Text],
}

impl<'a> Ask<'a> {
    pub fn new(opts: &'a [Text]) -> Self {
        Self { options: opts }
    }
}

impl Widget for Ask<'_> {
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
                    styled_text!(layout.gen_pos(), TextStyle { invert: true }, "{}", w.text)
                } else {
                    text!(layout.gen_pos(), "{}", w.text)
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

#[macro_export]
macro_rules! ask {
    ($($text:expr),*) => {{
        unsafe {
            let texts: &[$crate::ui::widget::text::Text] = &[
                $(
                    $crate::ui::widget::text::Text::new(
                        String::try_from($text).unwrap(),
                        (0, 0),
                        $crate::ui::widget::text::TextStyle { invert: false }
                    )
                ),*
            ];
            
            let options: &'static [$crate::ui::widget::text::Text] = ::core::mem::transmute(texts);
            Ask::new(options)
        }
    }};
}

