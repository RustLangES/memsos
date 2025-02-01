use crate::ui::layout::{Layout, LayoutArgs, LayoutParams};
use crate::ui::writer::get_ui;
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct VerticalLayout {
    y: AtomicUsize,
    x: usize,
    pub padding: usize,
    pub line_size: usize,
}

impl VerticalLayout {
    pub const fn new(params: LayoutParams) -> Self {
        let size = if let Some(val) = params.line_size {
            val
        } else {
            let ui = get_ui();
            ui.width()
        };
        Self {
            y: AtomicUsize::new(params.start_pos.1),
            x: params.start_pos.0,
            line_size: size,
            padding: params.padding,
        }
    }
}

impl Layout for VerticalLayout {
    fn spawn<T: super::LayoutChild>(&self, widget: &T) {
        let mut writer = get_ui();

        let (_, y) = self.gen_pos();

        widget.render_child(
            &mut writer,
            LayoutArgs {
                pos: (self.x, y),
                line_size: self.line_size,
            },
        );

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
