// Implementation of the Logger trait in core
use crate::ui::layout::{vertical::VerticalLayout, Layout};
use crate::ui::writer::get_ui;
use crate::{layout, text};
use memsos_core::Logger;
use crate::render;

pub struct DebugLogger<'a> {
    pub debug_layout: &'a VerticalLayout,
}

impl<'a> DebugLogger<'a> {
    pub fn new(layout: &'a VerticalLayout) -> Self {
        Self {
            debug_layout: layout,
        }
    }
}

impl Logger for DebugLogger<'_> {
    fn log(&self, message: core::fmt::Arguments<'_>) {
        layout!(self.debug_layout, &text!("{message}"));
    }
    fn ui_change_test(&self, test: &str) {
        let ui = get_ui();
        let test_text = text!((ui.width() - (ui.width() / 2) + 6, 50), "Actual test: {}", test);

        render!(
            &test_text
        );
    }
}
