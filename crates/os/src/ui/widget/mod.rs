pub mod input;
pub mod line;
pub mod text;
pub mod ask;

use crate::ui::writer::UiWriter;

pub trait Widget {
    fn render(&self, writer: &mut UiWriter);
    fn erase(&self, writer: &mut UiWriter);
}
