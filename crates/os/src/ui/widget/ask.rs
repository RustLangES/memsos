use crate::drivers::driver::Driver;
use crate::drivers::keyboard::{Key, KeyState, KEYBOARD};
use crate::ui::layout::{vertical::VerticalLayout, Layout, LayoutChild, LayoutParams};
use crate::ui::widget::text::TextStyle;
use crate::ui::widget::Widget;
use crate::{render, styled_text, text};
use core::sync::atomic::{AtomicUsize, Ordering};
use lang::DIALOGS;

pub struct Ask<'a> {
    pub options: &'a [&'static str],
    pub start_pos: (usize, usize),
    selection: AtomicUsize,
}

impl<'a> Ask<'a> {
    pub fn new(opts: &'a [&'static str], start_pos: (usize, usize)) -> Self {
        Self {
            options: opts,
            selection: AtomicUsize::new(0),
            start_pos,
        }
    }
    pub fn get_result(&self) -> &'static str {
        let index = self.selection.load(Ordering::SeqCst);

        let i = {
            if index > 0 {
                index - 1
            } else {
                index
            }
        };
        self.options[i]
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
                start_pos: self.start_pos,
                line_size: None,
            });
            let msg = text!(layout.gen_pos(), "{}", DIALOGS.ask.test_kind);
            render!(&msg);
            layout.margin(msg.spacing());
            for i in 0..self.options.len() {
                let w = &self.options[i];
                let t = {
                    if current == i {
                        styled_text!(layout.gen_pos(), TextStyle { invert: true }, "{}", w)
                    } else {
                        text!(layout.gen_pos(), "{}", w)
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

#[inline]
pub fn ask<'a>(options: &'a [&'static str], start_pos: (usize, usize)) -> Ask<'a> {
    Ask::new(options, start_pos)
}
