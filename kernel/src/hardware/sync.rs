//! A simple wrapper protecting its data using an atomic bool.
//!
//! The [Mutex] or [RwLock] is not available in core Rust.

use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

pub struct Protected<T> {
    mutex: AtomicBool,
    data: UnsafeCell<T>,
}
impl<T> Protected<T> {
    pub const fn new(data: T) -> Self {
        Protected {
            mutex: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }
    /// Calling lock will block until acquiring the lock.
    /// This can lead to a deadlock if the lock is not unlocked before/ while trying to lock,
    /// e.g. when calling lock twice without unlocking it.
    ///
    /// # Unlocking
    ///
    /// The lock is unlocked when the [ProtectedData] is dropped, e.g. when it leaves it's scope.
    pub fn lock(&self) -> ProtectedData<'_, T> {
        while self
            .mutex
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Acquire)
            .is_err()
        {}
        ProtectedData { protected: self }
    }
    fn unlock(&self) {
        self.mutex.store(false, Ordering::Release);
    }
    /// Required when a lock cannot be unlocked by dropping the [ProtectedData].
    /// Possible use-cases are an `mret` while still holding the lock or when printing a kernel panic.
    pub unsafe fn unsafe_unlock(&self) {
        self.unlock();
    }
}
/// Dropping the [ProtectedData] will unlock the [Protected].
pub struct ProtectedData<'a, T> {
    protected: &'a Protected<T>,
}
impl<T> ProtectedData<'_, T> {
    pub fn unlock(self) {}
}
impl<T> Drop for ProtectedData<'_, T> {
    fn drop(&mut self) {
        self.protected.unlock();
    }
}
impl<T> Deref for ProtectedData<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.protected.data.get() }
    }
}
impl<T> DerefMut for ProtectedData<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.protected.data.get() }
    }
}
unsafe impl<T> Sync for Protected<T> {}
