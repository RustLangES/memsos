#![no_std]
#![no_main]
#![allow(clippy::similar_names)]
#![feature(sync_unsafe_cell)]
#![feature(naked_functions)]

pub mod arch;
pub mod asm;
pub mod boot;
pub mod drivers;
pub mod mem;
pub mod request;
pub mod ui;

pub const PADDING: isize = 20;

pub enum StoreError {
    NoSpaceInBuffer,
}

pub struct StoreFb<'a> {
    pub buffer: [Option<&'a dyn crate::ui::widget::Widget>; 125],
    pub head: usize,
}

impl<'a> StoreFb<'a> {
    pub fn new() -> Self {
        Self {
            head: 0,
            buffer: [None; 125],
        }
    }
    pub fn push(&mut self, widget: &'a dyn crate::ui::widget::Widget) -> Result<(), StoreError> {
        if self.head == 125 {
            return Err(StoreError::NoSpaceInBuffer);
        }

        self.buffer[self.head] = Some(widget);

        self.head += 1;

        Ok(())
    }
    pub fn pop(&mut self) {
        if self.head == 0 {
            return;
        }

        self.buffer[self.head] = None;
        self.head -= 1;
    }
}

impl<'a> core::ops::Index<usize> for StoreFb<'a> {
    type Output = Option<&'a dyn crate::ui::widget::Widget>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.buffer[index]
    }
}
