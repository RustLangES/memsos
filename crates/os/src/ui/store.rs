use crate::ui::widget::Widget;
use core::ops::Index;

#[derive(Debug)]
pub enum StoreError {
    NoSpaceInBuffer,
}

pub struct StoreFb<'a> {
    pub buffer: [Option<&'a dyn Widget>; 125],
    pub head: usize,
}

impl<'a> StoreFb<'a> {
    pub fn new() -> Self {
        Self {
            head: 0,
            buffer: [None; 125],
        }
    }
    pub fn push(&mut self, widget: &'a dyn Widget) -> Result<(), StoreError> {
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

impl<'a> Index<usize> for StoreFb<'a> {
    type Output = Option<&'a dyn Widget>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.buffer[index]
    }
}
