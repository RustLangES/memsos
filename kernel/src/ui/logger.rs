// Implementation of the Logger trait in core
use memsos_core::Logger;
use crate::ui::layout::{vertical::VerticalLayout, Layout};
use crate::{text, layout};

pub struct DebugLogger<'a> {
    pub debug_layout: &'a VerticalLayout,
}

impl<'a> DebugLogger<'a> {
    pub fn new(layout: &'a VerticalLayout) -> Self {
        Self {
            debug_layout: layout
        }
    }
}

impl Logger for DebugLogger<'_> {
    fn log(&self, message: core::fmt::Arguments<'_>) {
        layout!(
            self.debug_layout,
            &text!("{message}")
        );
    }
}
