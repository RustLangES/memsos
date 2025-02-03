// Implementation of the Logger trait in core
use memsos_core::Logger;
use crate::ui::layout::{vertical::VerticalLayout, Layout};
use crate::{text, layout};

pub struct DebugLogger {
    pub debug_layout: VerticalLayout,
}

impl DebugLogger {
    pub fn new(layout: VerticalLayout) -> Self {
        Self {
            debug_layout: layout
        }
    }
}

impl Logger for DebugLogger {
    fn log(&self, message: core::fmt::Arguments<'_>) {
        layout!(
            self.debug_layout,
            &text!("{message}")
        );
    }
}
