use super::text::Text;
use super::Widget;
use crate::drivers::driver::Driver;
use crate::drivers::keyboard::{Key, KEYBOARD};
use crate::ui::layout::LayoutChild;

// Just a text wrapper that makes the program wait for the user to press space
pub struct Input<'a> {
    pub text: &'a Text,
}

impl Widget for Input<'_> {
    fn erase(&self, writer: &mut crate::ui::writer::UiWriter) {
        self.text.erase(writer);
    }
    fn render(&self, writer: &mut crate::ui::writer::UiWriter) {
        self.text.render(writer);

        KEYBOARD.wait_key(&Key::Space);
    }
}

impl LayoutChild for Input<'_> {
    fn spacing(&self) -> usize {
        self.text.spacing()
    }
    fn render_child(
        &self,
        writer: &mut crate::ui::writer::UiWriter,
        args: crate::ui::layout::LayoutArgs,
    ) {
        self.text.render_child(writer, args);

        KEYBOARD.read();
    }
}

#[inline]
pub fn input(text: &Text) -> Input {
    Input { text }
}
