pub struct MemoryMapping<'a, T> {
    address: usize,
    data: Option<Wrapper<'a, T>>,
}
struct Wrapper<'a, T>(&'a mut T);
impl<'a, T> MemoryMapping<'a, T> {
    pub const fn new(address: usize) -> Self {
        MemoryMapping {
            address,
            data: Option::None,
        }
    }
    #[must_use]
    pub fn get(&mut self) -> &mut T {
        self.data
            .get_or_insert(Wrapper(unsafe {
                (self.address as *mut T).as_mut().unwrap()
            }))
            .0
    }
}
