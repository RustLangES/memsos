// Implementation of the Logger trait in core
use crate::ui::layout::{vertical::VerticalLayout, Layout};
use crate::ui::writer::get_ui;
use crate::{layout, text};
use memsos_core::Logger;
use crate::{erase, render};
use crate::ui::widget::text::Text;

pub struct DebugLogger<'a> {
    pub debug_layout: &'a VerticalLayout,
    pub actual_test: Text
}

impl<'a> DebugLogger<'a> {
    pub fn new(layout: &'a VerticalLayout) -> Self {
        Self {
            debug_layout: layout,
            actual_test: text!(""),
        }
    }
}

impl Logger for DebugLogger<'_> {
    fn log(&self, message: core::fmt::Arguments<'_>) {
        layout!(self.debug_layout, &text!("{message}"));
    }
    fn ui_change_test(&mut self, test: &str) {
        let ui = get_ui();
        
        erase!(&self.actual_test);

        self.actual_test = text!((ui.width() - (ui.width() / 2) + 6, 50), "Actual test: {}", test);

        render!(
            &self.actual_test
        );
    }
}
