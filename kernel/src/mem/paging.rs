// https://github.com/anubis-rs/xernel/blob/main/kernel/src/mem/paging.rs

use crate::{
    mem::{HIGHER_HALF_OFFSET, KERNEL_OFFSET, frame::get_frame_allocator},
    println,
    requests::KERNEL_ADDRESS,
};
use core::{arch::asm, cell::SyncUnsafeCell, fmt::Write};
use x86_64::{
    PhysAddr, VirtAddr, align_down,
    structures::paging::{
        Page, PageSize, PageTable, PageTableFlags, PhysFrame, Size1GiB, Size2MiB, Size4KiB,
    },
};

pub static KERNEL_MAP: SyncUnsafeCell<Option<Pagemap>> = SyncUnsafeCell::new(None);

unsafe extern "C" {
    static _kernel_end: u64;
}

pub fn get_kernel_map() -> &'static mut Pagemap {
    unsafe { KERNEL_MAP.get().as_mut().unwrap().as_mut().unwrap() }
}

pub fn init_page_map() {
    unsafe {
        *KERNEL_MAP.get() = Some(Pagemap::new(None));
    }
}

#[derive(Debug)]
pub struct Pagemap {
    page_table: *mut PageTable,
}

impl Pagemap {
    pub fn new(pt_frame: Option<PhysFrame>) -> Self {
        let frame = pt_frame.unwrap_or_else(|| {
            let frame_allocator = get_frame_allocator();
            frame_allocator.alloc_frame()
        });

        let pt_address = unsafe {
            let ptr = (frame.start_address().as_u64() + *HIGHER_HALF_OFFSET) as *mut PageTable;

            *ptr = PageTable::new();

            if pt_frame.is_none() {
                (*ptr).zero();
            }

            ptr
        };

        Self {
            page_table: pt_address,
        }
    }
    pub fn pml4(&self) -> PhysAddr {
        PhysAddr::new(self.page_table as u64 - *HIGHER_HALF_OFFSET)
    }

    pub fn map_range(
        &mut self,
        phys: PhysAddr,
        virt: VirtAddr,
        amount: usize,
        flags: PageTableFlags,
        flush_tlb: bool,
    ) {
        assert!(u16::from(virt.page_offset()) == 0);
        assert!(phys.is_aligned(Size4KiB::SIZE));

        let aligned_amount = align_up(amount, Size4KiB::SIZE as usize);

        let mut offset: u64 = 0;

        let pages_4kb = (virt.align_up(Size2MiB::SIZE).as_u64() - virt.as_u64()) / Size4KiB::SIZE;

        for _ in 0..pages_4kb {
            if offset >= aligned_amount as u64 {
                break;
            }

            self.map::<Size4KiB>(
                PhysFrame::from_start_address(phys + offset).unwrap(),
                Page::from_start_address(virt + offset).unwrap(),
                flags,
                flush_tlb,
            );

            offset += Size4KiB::SIZE;
        }

        let pages_2mb = align_down(aligned_amount as u64 - offset, Size2MiB::SIZE) / Size2MiB::SIZE;

        for _ in 0..pages_2mb {
            self.map::<Size2MiB>(
                PhysFrame::from_start_address(phys + offset).unwrap(),
                Page::from_start_address(virt + offset).unwrap(),
                flags,
                flush_tlb,
            );

            offset += Size2MiB::SIZE;
        }

        let pages_4kb = align_up(aligned_amount - offset as usize, Size4KiB::SIZE as usize)
            / Size4KiB::SIZE as usize;

        for _ in 0..pages_4kb {
            self.map::<Size4KiB>(
                PhysFrame::from_start_address(phys + offset).unwrap(),
                Page::from_start_address(virt + offset).unwrap(),
                flags,
                flush_tlb,
            );

            offset += Size4KiB::SIZE;
        }
    }
    pub fn map<P: PageSize>(
        &mut self,
        phys: PhysFrame<P>,
        virt: Page<P>,
        flags: PageTableFlags,
        flush_tlb: bool,
    ) {
        let pml4 = self.page_table;
        let frame_allocator = get_frame_allocator();

        unsafe {
            let pml4_entry = &mut (&mut (*pml4))[virt.start_address().p4_index()];

            if !pml4_entry.flags().contains(PageTableFlags::PRESENT) {
                let frame = frame_allocator.alloc_frame::<Size4KiB>();
                let address = frame.start_address().as_u64();

                let p_table: *mut PageTable = (address + *HIGHER_HALF_OFFSET) as *mut PageTable;
                *p_table = PageTable::new();

                pml4_entry.set_addr(PhysAddr::new(address), flags);
            }

            pml4_entry.set_flags(pml4_entry.flags() | flags);

            let pml3 = (pml4_entry.addr().as_u64() + *HIGHER_HALF_OFFSET) as *mut PageTable;
            let pml3_entry = &mut (&mut (*pml3))[virt.start_address().p3_index()];

            if P::SIZE == Size1GiB::SIZE {
                assert!(
                    u16::from(virt.start_address().p2_index()) == 0
                        && u16::from(virt.start_address().p1_index()) == 0
                        && u16::from(virt.start_address().page_offset()) == 0
                );

                pml3_entry.set_addr(phys.start_address(), flags | PageTableFlags::HUGE_PAGE);

                if flush_tlb {
                    flush(virt.start_address());
                }

                return;
            }

            if !pml3_entry.flags().contains(PageTableFlags::PRESENT) {
                let frame = frame_allocator.alloc_frame::<Size4KiB>();
                let address = frame.start_address().as_u64();
                let p_table: *mut PageTable = (address + *HIGHER_HALF_OFFSET) as *mut PageTable;
                *p_table = PageTable::new();

                pml3_entry.set_addr(PhysAddr::new(address), flags);
            }

            pml3_entry.set_flags(pml4_entry.flags() | flags);

            let pml2 = (pml3_entry.addr().as_u64() + *HIGHER_HALF_OFFSET) as *mut PageTable;

            let pml2_entry = &mut (&mut (*pml2))[virt.start_address().p2_index()];

            if P::SIZE == Size2MiB::SIZE {
                assert!(
                    u16::from(virt.start_address().p1_index()) == 0
                        && u16::from(virt.start_address().page_offset()) == 0
                );

                pml2_entry.set_addr(phys.start_address(), flags | PageTableFlags::HUGE_PAGE);

                if flush_tlb {
                    flush(virt.start_address());
                }

                return;
            }

            if !pml2_entry.flags().contains(PageTableFlags::PRESENT) {
                let frame = frame_allocator.alloc_frame::<Size4KiB>();

                let address = frame.start_address().as_u64();

                let p_table: *mut PageTable = (address + *HIGHER_HALF_OFFSET) as *mut PageTable;
                *p_table = PageTable::new();

                pml2_entry.set_addr(PhysAddr::new(address), flags);
            }

            pml2_entry.set_flags(pml4_entry.flags() | flags);

            let pml1 = (pml2_entry.addr().as_u64() + *HIGHER_HALF_OFFSET) as *mut PageTable;

            let pml1_entry = &mut (&mut (*pml1))[virt.start_address().p1_index()];

            pml1_entry.set_addr(phys.start_address(), flags);

            if flush_tlb {
                flush(virt.start_address());
            }
        }
    }
    pub fn kernel_map(&mut self) {
        let kernel_address = KERNEL_ADDRESS.get_response().unwrap();
        let kenel_base_addr = kernel_address.physical_base();
        let kernel_virt_addr = kernel_address.virtual_base();

        println!("Kernel base addr: {:x}", kenel_base_addr);
        println!("Kernel virt addr: {:x}", kernel_virt_addr);

        let kernel_size = unsafe { ((&_kernel_end as *const u64) as u64) - kernel_virt_addr };
        println!("Kernel size: {}", kernel_size);

        self.map_range(
            PhysAddr::new(kenel_base_addr),
            VirtAddr::new(KERNEL_OFFSET),
            kernel_size as usize,
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
            false,
        );
    }
}

fn flush(addr: VirtAddr) {
    unsafe {
        core::arch::asm!("invlpg [{}]", in(reg) addr.as_u64(), options(nostack, preserves_flags));
    }
}

unsafe impl Sync for Pagemap {}
unsafe impl Send for Pagemap {}

pub fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}
