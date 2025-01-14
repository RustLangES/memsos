use crate::ui::layout::{Layout, LayoutArgs};
use crate::ui::writer::get_ui;
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct VerticalLayout {
    pub y: AtomicUsize,
    pub padding: (usize, usize),
}

impl VerticalLayout {
    pub const fn new(padding: (usize, usize), start_pos: usize) -> Self {
        Self {
            y: AtomicUsize::new(start_pos),
            padding,
        }
    }
}

impl Layout for VerticalLayout {
    fn spawn<T: super::LayoutChild>(&self, widget: &T) {
        let mut writer = get_ui();

        self.y.fetch_add(self.padding.1, Ordering::SeqCst);

        let y = self.y.load(Ordering::SeqCst);

        let pos = widget.render_child(
            &mut writer,
            LayoutArgs {
                pos: (self.padding.0, y),
            },
        );

        self.y.fetch_add(widget.spacing() + pos.1, Ordering::SeqCst);
    }
}
