use core::{alloc::GlobalAlloc, mem::MaybeUninit, ptr::NonNull};
use linked_list_allocator::Heap;
use spin::Mutex;

const HEAP_SIZE: usize = 1024;

pub struct Allocator {
    pub heap: Mutex<Heap>,
}

static mut TEMP_ARRAY: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

impl Allocator {
    pub const fn new() -> Self {
        let heap = Heap::empty();
        Self {
            heap: Mutex::new(heap),
        }
    }
    pub fn init(&self) {
        let mut heap = self.heap.lock();
        unsafe {
            #[allow(static_mut_refs)]
            (*heap).init_from_slice(&mut TEMP_ARRAY);
        }
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let mut heap = self.heap.lock();
        (*heap)
            .allocate_first_fit(layout)
            .expect("Cannot allocate memory")
            .as_ptr()
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let mut heap = self.heap.lock();

        (*heap).deallocate(NonNull::new_unchecked(ptr), layout);
    }
}
