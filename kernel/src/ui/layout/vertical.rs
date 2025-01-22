use crate::ui::layout::{Layout, LayoutArgs};
use crate::ui::writer::get_ui;
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct VerticalLayout {
    y: AtomicUsize,
    x: usize,
    pub padding: usize,
}

impl VerticalLayout {
    pub const fn new(start_pos: (usize, usize), padding: usize) -> Self {
        Self {
            y: AtomicUsize::new(start_pos.1),
            x: start_pos.0,
            padding,
        }
    }
}

impl Layout for VerticalLayout {
    fn spawn<T: super::LayoutChild>(&self, widget: &T) {
        let mut writer = get_ui();

        let (_, y) = self.gen_pos();

        widget.render_child(&mut writer, LayoutArgs { pos: (self.x, y) });

        self.y.fetch_add(widget.spacing(), Ordering::SeqCst);
    }
    fn gen_pos(&self) -> (usize, usize) {
        self.y.fetch_add(self.padding, Ordering::SeqCst);

        let y = self.y.load(Ordering::SeqCst);

        (self.x, y)
    }
    fn margin(&self, size: usize) {
        self.y.fetch_add(size, Ordering::SeqCst);
    } 
}
