use crate::ui::widget::Widget;

pub trait Layout {
    fn spawn<T: Widget>(&self, widget: &T);
}
