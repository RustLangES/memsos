use crate::ui::writer::UiWriter;

pub trait Widget {
    fn render(&self, writer: &mut UiWriter);
}
