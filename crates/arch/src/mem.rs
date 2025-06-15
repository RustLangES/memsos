pub fn read(addr: usize) -> usize {
    let ptr = addr as *const u64;

    unsafe { ptr.read() as usize }
}

pub fn write(addr: usize, value: usize) {
    let ptr = addr as *mut u64;

    unsafe { ptr.write(value as u64); }
}
