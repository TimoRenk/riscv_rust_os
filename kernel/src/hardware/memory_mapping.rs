//! A memory mapping to read any type T from a specified memory location.

pub struct MemoryMapping<T> {
    val: *mut T,
}
impl<T> MemoryMapping<T> {
    pub const fn new(address: usize) -> Self {
        MemoryMapping {
            val: address as *mut T,
        }
    }
    pub unsafe fn read(&self) -> T {
        self.val.read_volatile()
    }
    pub unsafe fn write(&self, val: T) {
        self.val.write_volatile(val);
    }
}
