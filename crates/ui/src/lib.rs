#![no_std]
#![feature(sync_unsafe_cell)]

pub mod components;
pub mod sections;

use core::cell::SyncUnsafeCell;

use fb::{display::FbDisplay, get_ui_writer};

use crate::sections::test_info::TestInfoSection;

pub static UI_STATE: SyncUnsafeCell<Option<UiState>> = SyncUnsafeCell::new(None);

pub fn init_ui_state(state: UiState) {
    unsafe {
        *UI_STATE.get() = Some(state);
    }
}

/// # Panics
///
///  It may cause panic if this function is called before the ui state is initialized.
pub fn get_ui_state() -> &'static mut UiState {
    unsafe { UI_STATE.get().as_mut().unwrap().as_mut().unwrap() }
}

#[inline]
pub fn render_ui_state() {
    get_ui_state().render_all();
}

pub struct UiState {
    pub test_info_section: TestInfoSection,
}

impl UiState {
    pub fn render_all(&mut self) {
        self.test_info_section.render(get_ui_writer());
    }
}

pub trait Section {
    fn render(&mut self, screen: &mut FbDisplay);
}

pub trait RenderSection {
    fn render_section<T: Section>(&mut self, section: &mut T);
}

impl RenderSection for FbDisplay {
    fn render_section<T: Section>(&mut self, section: &mut T) {
        section.render(self);
    }
}

#[inline]
pub fn render_section<T: Section>(section: &mut T) {
    get_ui_writer().render_section(section);
}

#[macro_export]
macro_rules! push_logs {
    ($($arg:tt)*) => {{
        let mut val = &mut get_ui_state().test_info_section.logs;

        write!(val, "{}", format_args!($($arg)*)).expect("Cannot format args");
    }};
}
