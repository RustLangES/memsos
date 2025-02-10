use crate::drivers::driver::Driver;
use crate::drivers::keyboard::{Key, KeyState, KEYBOARD};
use crate::ui::layout::{vertical::VerticalLayout, Layout, LayoutChild, LayoutParams};
use crate::ui::widget::text::{Text, TextStyle};
use crate::ui::widget::Widget;
use crate::{render, styled_text, text};
use core::sync::atomic::{AtomicUsize, Ordering};
use heapless::String;

pub struct Ask<'a> {
    pub options: &'a [Text],
    selection: AtomicUsize,
}

impl<'a> Ask<'a> {
    pub fn new(opts: &'a [Text]) -> Self {
        Self {
            options: opts,
            selection: AtomicUsize::new(0),
        }
    }
    pub fn get_result(&self) -> String<256> {
        let index = self.selection.load(Ordering::SeqCst);

        let i = {
            if index > 0 {
                index - 1
            } else {
                index
            }
        };
        self.options[i].text.clone()
    }
}

impl Widget for Ask<'_> {
    fn render(&self, _writer: &mut crate::ui::writer::UiWriter) {
        let mut current = 0;
        let mut read = Key::Unknown(0);

        while !(read == Key::Space) {
            if current >= self.options.len() {
                current = 0;
            }
            match read {
                Key::Down => {
                    if current <= self.options.len() {
                        current += 1;
                    }
                }
                Key::Up => {
                    if current > 0 {
                        current = current.saturating_sub(1);
                    }
                }
                _ => {}
            }

            let layout = VerticalLayout::new(LayoutParams {
                max_y: None,
                padding: 0,
                start_pos: (0, 0),
                line_size: None,
            });
            let msg = text!(layout.gen_pos(), "What kind of tests do you want?");
            render!(&msg);
            layout.margin(msg.spacing());
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
            let e = KEYBOARD.read();
            if e.state == KeyState::Release {
                read = e.key;
            }
        }
        self.selection.store(current, Ordering::SeqCst);
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
