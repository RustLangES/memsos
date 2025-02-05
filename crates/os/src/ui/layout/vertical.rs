use crate::ui::layout::{Layout, LayoutArgs, LayoutParams};
use crate::ui::writer::get_ui;
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct VerticalLayout {
    y: AtomicUsize,
    pub line_size: usize,
    pub max_y: usize,
    params: LayoutParams,
}

impl VerticalLayout {
    pub const fn new(params: LayoutParams) -> Self {
        let size = if let Some(val) = params.line_size {
            val
        } else {
            let ui = get_ui();
            ui.width()
        };
        let max_y = if let Some(val) = params.max_y {
            val
        } else {
            let ui = get_ui();
            ui.height()
        };

        Self {
            y: AtomicUsize::new(params.start_pos.1),
            line_size: size,
            max_y,
            params,
        }
    }
}

impl Layout for VerticalLayout {
    fn spawn<T: super::LayoutChild>(&self, widget: &T) {
        let mut writer = get_ui();

        let (_, y) = self.gen_pos();
        let new_y = y + widget.spacing();

        if new_y >= self.max_y {
            writer.clear_zone(
                (self.params.start_pos.0, self.params.start_pos.1),
                (self.params.start_pos.0 + self.line_size, self.max_y),
            );
            self.y.store(self.params.start_pos.1, Ordering::SeqCst);
            return;
        }

        widget.render_child(
            &mut writer,
            LayoutArgs {
                pos: (self.params.start_pos.0, y),
                line_size: self.line_size,
            },
        );

        self.y.store(new_y, Ordering::SeqCst);
    }

    fn gen_pos(&self) -> (usize, usize) {
        self.y.fetch_add(self.params.padding, Ordering::SeqCst);

        let y = self.y.load(Ordering::SeqCst);

        (self.params.start_pos.0, y)
    }

    fn margin(&self, size: usize) {
        self.y.fetch_add(size, Ordering::SeqCst);
    }
}
