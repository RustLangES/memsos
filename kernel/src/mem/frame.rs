use alloc::vec::Vec;
use core::cell::SyncUnsafeCell;
use x86_64::{
    PhysAddr,
    structures::paging::{PageSize, PhysFrame},
};

pub static FRAME_ALLOCATOR: SyncUnsafeCell<Option<PhysFrameAllocator>> = SyncUnsafeCell::new(None);

pub fn init_frame_allocator(start: u64) {
    unsafe {
        *FRAME_ALLOCATOR.get() = Some(PhysFrameAllocator::new(start));
    }
}

pub fn get_frame_allocator() -> &'static mut PhysFrameAllocator {
    unsafe { FRAME_ALLOCATOR.get().as_mut().unwrap().as_mut().unwrap() }
}

pub struct FreeFrame {
    pub start_addr: PhysAddr,
    pub size: u64,
}

pub struct PhysFrameAllocator {
    free_list: Vec<FreeFrame>,
    pointer: u64,
}

impl PhysFrameAllocator {
    pub const fn new(start: u64) -> Self {
        Self {
            free_list: Vec::new(),
            pointer: start,
        }
    }
    pub fn alloc_frame<P: PageSize>(&mut self) -> PhysFrame<P> {
        if !self.free_list.is_empty()
            && let Some(index) = self.free_list.iter().position(|f| f.size == P::SIZE)
        {
            let f = self.free_list.remove(index);
            return PhysFrame::from_start_address(f.start_addr).unwrap();
        }

        let addr = PhysAddr::new(self.pointer);
        self.pointer += P::SIZE;

        PhysFrame::from_start_address(addr).unwrap()
    }
    pub fn free_frame<P: PageSize>(&mut self, frame: PhysFrame<P>) {
        self.free_list.push(FreeFrame {
            start_addr: frame.start_address(),
            size: frame.size(),
        });
    }
}
