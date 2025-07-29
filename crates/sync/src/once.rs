#![allow(clippy::new_without_default, clippy::missing_panics_doc)]

use core::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    ops::Deref,
    sync::atomic::{AtomicBool, Ordering},
};

pub struct Once<T> {
    val: UnsafeCell<MaybeUninit<T>>,
    is_set: AtomicBool,
}

impl<T> Once<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            is_set: AtomicBool::new(false),
            val: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }
    pub fn has_value(&self) -> bool {
        self.is_set.load(Ordering::Relaxed)
    }
    pub fn call_once<F>(&self, func: F)
    where
        F: FnOnce() -> T,
    {
        assert!(
            !self.is_set.load(Ordering::Acquire),
            "Struct Once can only be set once."
        );

        unsafe {
            let val = func.call_once(());
            (*self.val.get()).as_mut_ptr().write(val);
        }

        self.is_set.store(true, Ordering::Release);
    }
}

unsafe impl<T> Sync for Once<T> {}
unsafe impl<T> Send for Once<T> {}

impl<T> Deref for Once<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        assert!(
            self.is_set.load(Ordering::Acquire),
            "The value cannot be accessed if it has not yet been initialized"
        );

        unsafe { &*(*self.val.get()).as_ptr() }
    }
}
