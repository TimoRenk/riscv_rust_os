pub struct RegisterMapping<T: 'static> {
    address: usize,
    register: Option<Wrapper<T>>,
}
struct Wrapper<T: 'static>(&'static mut T);
impl<T> RegisterMapping<T> {
    pub const fn new(address: usize) -> Self {
        RegisterMapping {
            address,
            register: Option::None,
        }
    }
    #[must_use]
    pub fn get(&mut self) -> &mut T {
        self.register
            .get_or_insert(Wrapper(unsafe {
                (self.address as *mut T).as_mut().unwrap()
            }))
            .0
    }
}
