use crate::ui::widget::Widget;
use crate::ui::writer::UiWriter;

pub struct LayoutArgs {
    pub pos: (usize, usize),
    pub line_size: usize,
}

#[derive(Debug)]
pub struct LayoutParams {
    pub padding: usize,
    pub start_pos: (usize, usize),
    pub line_size: Option<usize>, // If None line_size would be writer.x
    pub max_y: Option<usize>,     // If None max_pos would be writer.y
}

pub trait Layout {
    fn spawn<T: LayoutChild + Widget>(&self, widget: &T);
    fn gen_pos(&self) -> (usize, usize);
    fn margin(&self, size: usize);
}

pub trait LayoutChild {
    fn render_child(&self, writer: &mut UiWriter, args: LayoutArgs);
    fn spacing(&self) -> usize;
}
