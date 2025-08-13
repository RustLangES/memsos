use core::time::Duration;

use alloc::vec::Vec;
use core::fmt::Write;

use limine::memory_map::{Entry, EntryType};
use sync::Once;
use ui::get_ui_state;

pub type MemoryMap = &'static [&'static Entry];

pub static MEMORY_MAP: Once<MemoryMap> = Once::new();

#[derive(Debug, Clone, Copy)]
pub struct MemoryEntry {
    pub start: u64,
    pub end: u64,
}

pub trait MemModule {
    const NAME: &'static str;

    fn init(memory_map: MemoryMap) -> Self;
    fn run(&mut self, reports: &mut Vec<MemoryReport>);
}

#[derive(Debug)]
pub enum MemoryError {
    StuckAt,
    BitFade(Duration),
}

#[derive(Debug)]
pub struct MemoryReport {
    pub address: usize,
    pub kind: MemoryError,
}

pub fn init_mem_module(memory: MemoryMap) {
    MEMORY_MAP.call_once(|| memory);
}

pub fn load_memtest<T: MemModule>(reports: &mut Vec<MemoryReport>) {
    get_ui_state()
        .test_info_section
        .set_current_test(heapless::format!("Running test {}", T::NAME).get());

    let mut test = T::init(*MEMORY_MAP);
    test.run(reports);
}

#[must_use]
pub fn is_usable_memory(entry: &Entry) -> bool {
    entry.entry_type == EntryType::USABLE
}
