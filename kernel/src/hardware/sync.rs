use core::sync::atomic::{AtomicBool, Ordering};
pub struct Protected<T> {
    mutex: AtomicBool,
    data: T,
}
impl<T> Protected<T> {
    pub const fn new(data: T) -> Self {
        Protected {
            mutex: AtomicBool::new(false),
            data,
        }
    }
    pub fn lock_and_get(&mut self) -> &mut T {
        while self
            .mutex
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Acquire)
            .is_err()
        {}
        return &mut self.data;
    }
    pub fn unlock(&mut self) {
        self.mutex.store(false, Ordering::Release);
    }
}
