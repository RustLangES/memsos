pub fn memread(ptr: *mut u64) -> u64 {
    let value = unsafe { ptr.read() };
    value
}
