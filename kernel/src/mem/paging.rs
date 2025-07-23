// https://github.com/anubis-rs/xernel/blob/main/kernel/src/mem/paging.rs

use crate::mem::{KERNEL_OFFSET, frame::get_frame_allocator};
use boot::HIGHER_HALF_OFFSET;
use boot::requests::KERNEL_ADDRESS;
use x86_64::{
    VirtAddr,
    registers::control::Cr3,
    structures::paging::{
        Mapper, OffsetPageTable, Page, PageSize, PageTable, PageTableFlags, PhysFrame,
    },
};

use core::{cell::SyncUnsafeCell, fmt::Write};

pub static KERNEL_MAP: SyncUnsafeCell<Option<OffsetPageTable<'static>>> = SyncUnsafeCell::new(None);

pub fn init_page_map() {
    unsafe {
        *KERNEL_MAP.get() = Some(get_mapper());
    }
}

pub fn get_kernel_map() -> &'static mut OffsetPageTable<'static> {
    unsafe { KERNEL_MAP.get().as_mut().unwrap().as_mut().unwrap() }
}

pub fn map<P: PageSize + core::fmt::Debug>(
    page: Page<P>,
    frame: PhysFrame<P>,
    flags: PageTableFlags,
    flush_tlb: bool,
) where
    OffsetPageTable<'static>: Mapper<P>,
{
    let frame_allocator = get_frame_allocator();

    unsafe {
        let mapper = get_kernel_map();
        let map = mapper
            .map_to(page, frame, flags, frame_allocator)
            .expect("Cannot map page");

        if flush_tlb {
            map.flush();
            return;
        }

        map.ignore();
    }
}

fn get_mapper() -> OffsetPageTable<'static> {
    let addr = VirtAddr::new(*HIGHER_HALF_OFFSET);
    unsafe { OffsetPageTable::new(get_page_table(addr), addr) }
}

fn get_page_table(virt_addr: VirtAddr) -> &'static mut PageTable {
    let (frame, _) = Cr3::read();
    let phys_addr = frame.start_address().as_u64();
    let virt = virt_addr.as_u64() + phys_addr;
    unsafe { &mut *(virt as *mut PageTable) }
}

pub fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}
