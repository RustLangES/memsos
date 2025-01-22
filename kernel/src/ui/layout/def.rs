use crate::ui::writer::UiWriter;

pub struct LayoutArgs {
    pub pos: (usize, usize),
}

pub trait Layout {
    fn spawn<T: LayoutChild>(&self, widget: &T);
    fn gen_pos(&self) -> (usize, usize);
    fn margin(&self, size: usize);
}

pub trait LayoutChild {
    fn render_child(&self, writer: &mut UiWriter, args: LayoutArgs) -> (usize, usize);
    fn spacing(&self) -> usize;
}
