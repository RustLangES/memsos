#[inline]
pub fn memread(addr: u64) -> Result<u64, super::MemError> {
    crate::mem::MEMORY_WRITER.read(addr)
}
