use core::cell::SyncUnsafeCell;
use core::ops::{Deref, DerefMut};

use embedded_graphics::mono_font;
use embedded_graphics::prelude::Size;
use kolibri_embedded_gui::label::Label;
use kolibri_embedded_gui::style::{Spacing, Style};

use bootloader_api::info::FrameBufferInfo;
use kolibri_embedded_gui::ui::Ui;

mod color;
mod writer;

pub use color::Color;
pub use writer::MemsosUIWriter;

use crate::drivers::keyboard::{Key, Scanner};
use crate::power::reboot::reboot;

pub static UI: SyncUnsafeCell<Option<MemsosUI<'static>>> = SyncUnsafeCell::new(None);

pub fn memsos_ui_style() -> Style<Color> {
    Style {
        background_color: Color::RGB_BLACK,
        item_background_color: Color::RGB_BLACK,
        highlight_item_background_color: Color::RGB_BLACK,
        border_color: Color::RGB_GREEN,
        highlight_border_color: Color::RGB_GREEN,
        primary_color: Color::RGB_GREEN,
        secondary_color: Color::RGB_YELLOW,
        icon_color: Color::RGB_GREEN,
        text_color: Color::RGB_GREEN,
        default_widget_height: 17,
        border_width: 1,
        highlight_border_width: 3,
        default_font: mono_font::ascii::FONT_10X20,
        spacing: Spacing {
            item_spacing: Size::new(8, 4),
            button_padding: Size::new(5, 5),
            default_padding: Size::new(1, 1),
            window_border_padding: Size::new(3, 3),
        },
    }
}

pub struct MemsosUI<'a> {
    info: FrameBufferInfo,
    ui: Ui<'a, MemsosUIWriter<Color>, Color>,
}

unsafe impl<'a> Send for MemsosUI<'a> {}
unsafe impl<'a> Sync for MemsosUI<'a> {}

impl<'a> MemsosUI<'a> {
    pub fn new(buffer: &'a mut MemsosUIWriter<Color>, info: FrameBufferInfo) -> Self {
        let ui = Ui::new_fullscreen(buffer, memsos_ui_style());

        Self { info, ui }
    }

    pub fn clear(&mut self) {
        self.ui.clear_background().unwrap();
    }

    pub fn label(&mut self, text: &str) {
        self.ui.add(Label::new(text));
    }

    pub fn show_alert(&mut self, title: &str, message: &str) {
        self.ui.add(Label::new(title));
        self.ui.add(Label::new(message));
    }

    pub fn show_alert_unrecoverable(&mut self, title: &str, message: &str) -> ! {
        self.ui.add(Label::new(title));
        self.ui.add(Label::new(message));

        let scanner = Scanner;
        _ = self.ui.allocate_space(Size::new(10, 20));
        self.ui
            .add(Label::new("Press space to reboot your computer!"));
        scanner.wait_for_key(Key::Space);
        reboot();

        loop {}
    }
}

impl<'a> Deref for MemsosUI<'a> {
    type Target = Ui<'a, MemsosUIWriter<Color>, Color>;

    fn deref(&self) -> &Self::Target {
        &self.ui
    }
}

impl<'a> DerefMut for MemsosUI<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ui
    }
}

#[macro_export]
macro_rules! init_ui {
    ($buffer: expr, $info: expr) => {
        unsafe {
            let buff = core::mem::transmute::<
                &'_ mut $crate::ui::MemsosUIWriter<$crate::ui::Color>,
                &'static mut $crate::ui::MemsosUIWriter<$crate::ui::Color>,
            >(&mut MemsosUIWriter::new(
                $info.width.try_into().unwrap(),
                $info.height.try_into().unwrap(),
                $info.stride.try_into().unwrap(),
                $info.bytes_per_pixel.try_into().unwrap(),
                $buffer,
            ));
            (*UI.get()).get_or_insert_with(|| MemsosUI::new(buff, $info))
        }
    };
}

#[macro_export]
macro_rules! get_ui {
    () => {{
        let ui = unsafe { (*$crate::ui::UI.get()).as_mut().unwrap() };
        ui.clear();
        ui
    }};
}
