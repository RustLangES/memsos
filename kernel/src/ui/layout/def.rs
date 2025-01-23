use crate::ui::widget::Widget;
use crate::ui::writer::UiWriter;

pub struct LayoutArgs {
    pub pos: (usize, usize),
    pub line_size: usize,
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
