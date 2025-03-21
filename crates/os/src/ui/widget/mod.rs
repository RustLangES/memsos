pub mod ask;
pub mod input;
pub mod line;
pub mod text;

use crate::ui::layout::ChildArgs;
use crate::ui::writer::UiWriter;

pub trait Widget {
    fn spacing(&self) -> usize;
    fn render_as_child(&self, writer: &mut UiWriter, args: ChildArgs);
    fn render(&self, writer: &mut UiWriter);
    fn erase(&self, writer: &mut UiWriter);
}
