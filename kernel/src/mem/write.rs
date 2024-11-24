pub fn memwrite(ptr: *mut u64, value: u64) {
    unsafe {
        *ptr = value;
    }
}
