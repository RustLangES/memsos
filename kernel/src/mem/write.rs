pub fn memwrite(addr: u64, value: u64) -> Result<(), super::MemError> {
    crate::mem::MEMORY_WRITER.write(addr, value)
}
