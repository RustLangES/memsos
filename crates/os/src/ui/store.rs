use crate::ui::layout::ChildArgs;
use crate::ui::widget::Widget;
use core::ops::Index;

#[derive(Debug)]
pub enum StoreError {
    NoSpaceInBuffer,
}

pub struct WidgetWrapper<'a> {
    pub child_args: Option<ChildArgs>,
    pub widget: &'a dyn Widget,
}

impl Widget for WidgetWrapper<'_> {
    fn erase(&self, writer: &mut crate::ui::writer::UiWriter) {
        self.widget.erase(writer);
    }
    fn spacing(&self) -> usize {
        self.widget.spacing()
    }
    fn render(&self, writer: &mut crate::ui::writer::UiWriter) {
        if let Some(args) = self.child_args {
            self.widget.render_as_child(writer, args);
        } else {
            self.widget.render(writer);
        }
    }
    // this widget is not designed to run on layouts
    fn render_as_child(&self, _writer: &mut crate::ui::writer::UiWriter, _args: ChildArgs) {
        unimplemented!();
    }
}

pub struct StoreFb<'a> {
    pub buffer: [Option<WidgetWrapper<'a>>; 125],
    pub head: usize,
}

impl<'a> StoreFb<'a> {
    pub fn new() -> Self {
        Self {
            head: 0,
            buffer: [const { None }; 125],
        }
    }
    pub fn push(
        &mut self,
        widget: &'a dyn Widget,
        child_args: Option<ChildArgs>,
    ) -> Result<(), StoreError> {
        if self.head == 125 {
            return Err(StoreError::NoSpaceInBuffer);
        }

        self.buffer[self.head] = Some(WidgetWrapper { widget, child_args });

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
    type Output = Option<WidgetWrapper<'a>>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.buffer[index]
    }
}
