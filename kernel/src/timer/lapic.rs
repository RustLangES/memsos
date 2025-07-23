use crate::mem::paging::get_kernel_map;
use arch::rdmsr;
use boot::HIGHER_HALF_OFFSET;
use core::fmt::Write;
use fb::println;
use x86_64::{
    structures::paging::{Page, PageTableFlags, PhysFrame, Size2MiB, Size4KiB}, PhysAddr, VirtAddr
};

use alloc::collections::VecDeque;
use lazy_static::lazy_static;

use x86_64::{
    registers::control::Cr3,
    structures::paging::{
        FrameAllocator, FrameDeallocator, Mapper, OffsetPageTable, PageSize, PageTable,
    },
};

// Ignore this code, is only from tests
const KERNEL_MEM_OFFSET: u64 = 0xFFFF_8000_0000_0000;
use spin::Mutex;

lazy_static! {
    pub static ref MemMapper: Mutex<(PhysAlloc, OffsetPageTable<'static>)> =
        Mutex::new((PhysAlloc::new(0x10000), get_mapper()));
}

#[derive(Clone)]
pub struct PhysAlloc {
    pointer: u64,
    free_list: VecDeque<PhysFrame<Size4KiB>>,
}

impl PhysAlloc {
    pub fn new(pointer: u64) -> Self {
        PhysAlloc {
            pointer,
            free_list: VecDeque::new(),
        }
    }
}

unsafe impl FrameAllocator<Size4KiB> for PhysAlloc {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        if let Some(frame) = self.free_list.pop_front() {
            return Some(frame);
        }

        let addr = PhysAddr::new(self.pointer);
        let frame = PhysFrame::containing_address(addr);

        self.pointer += Size4KiB::SIZE;
        Some(frame)
    }
}

impl FrameDeallocator<Size4KiB> for PhysAlloc {
    unsafe fn deallocate_frame(&mut self, frame: PhysFrame<Size4KiB>) {
        self.free_list.push_front(frame);
    }
}


pub fn map_phys_to_virt(phys: PhysAddr) -> VirtAddr {
    let phys_u64 = phys.as_u64();
    assert!(phys_u64 % Size4KiB::SIZE == 0);
    VirtAddr::new(phys_u64 + KERNEL_MEM_OFFSET)
}

pub fn map_addr(addr: u64) {
    let mut mem_mapper_guard = MemMapper.try_lock().expect("Cannot get memory map");

    let page: Page<Size4KiB> = Page::containing_address(VirtAddr::new(addr + *HIGHER_HALF_OFFSET));
    let frame: PhysFrame<Size4KiB> = PhysFrame::from_start_address(PhysAddr::new(addr)).unwrap();
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_EXECUTE;

    let (allocator, mapper) = &mut *mem_mapper_guard;

    unsafe {
        mapper
            .map_to(page, frame, flags, allocator)
            .expect("Cannot map addr")
            .flush();
    }
}

fn get_mapper() -> OffsetPageTable<'static> {
    let addr = VirtAddr::new(KERNEL_MEM_OFFSET);
    unsafe { OffsetPageTable::new(get_page_table(addr), addr) }
}

pub fn get_page_table(virt_addr: VirtAddr) -> &'static mut PageTable {
    let (frame, _) = Cr3::read();
    let phys_addr = frame.start_address().as_u64();
    let virt = virt_addr.as_u64() + phys_addr;
    unsafe { &mut *(virt as *mut PageTable) }
}

const APIC_BASE_MSR: u32 = 0x1B;

pub struct LocalApic {
    address: u64,
    frequency: u64,
}

impl LocalApic {
    pub fn new() -> Self {
        let mut apic_base = rdmsr(APIC_BASE_MSR);
       // let mut mapper = get_kernel_map();

        apic_base &= !(1 << 8);
        apic_base &= !(1 << 11);

        println!("Apic base: 0x{:x}", apic_base);

        map_addr(apic_base);

        let mut lapic = LocalApic {
            address: apic_base + *HIGHER_HALF_OFFSET,
            frequency: 0,
        };

        lapic.enable_apic();

        lapic
    }
    pub fn read(&self, reg: u64) -> u32 {
        unsafe { ((self.address + reg) as *const u32).read_volatile() }
    }
    pub fn write(&self, reg: u64, val: u32) {
        unsafe {
            ((self.address + reg) as *mut u32).write_volatile(val);
        }
    }
    pub fn siv(&self) -> u32 {
        self.read(LapicReg::SPURIOUS)
    }
    pub fn set_siv(&self, val: u32) {
        self.write(LapicReg::SPURIOUS, val);
    }
    pub fn enable_apic(&self) {
        self.set_siv(0x1ff);

        self.write(LapicReg::TPR, 0);
    }
}

pub struct LapicReg;

impl LapicReg {
    pub const SPURIOUS: u64 = 0xF0;
    pub const TPR: u64 = 0x80;
}
