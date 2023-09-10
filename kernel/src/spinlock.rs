use core::{cell::UnsafeCell, sync::atomic::{AtomicBool, Ordering}, hint, ops::Deref, ops::DerefMut};

pub struct SpinLock<T: ?Sized> {
    locked: AtomicBool,
    data: UnsafeCell<T>
}

unsafe impl<T: ?Sized + Send> Send for SpinLock<T> {}
unsafe impl<T: ?Sized + Send> Sync for SpinLock<T> {}

pub struct SpinLockGuard<'a, T: ?Sized + 'a> {
    lock: &'a SpinLock<T>
}

impl<T: ?Sized> !Send for SpinLockGuard<'_, T> {}
unsafe impl<T: ?Sized + Sync> Sync for SpinLockGuard<'_, T> {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self { locked: AtomicBool::new(false), data: UnsafeCell::new(value) }
    }
}

impl<T: ?Sized> SpinLock<T> {
    pub fn is_locked(&self) -> bool {
        self.locked.load(Ordering::Acquire)
    }

    pub fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }

    pub fn try_lock(&self) -> Result<SpinLockGuard<T>, ()> {
        if self.locked.compare_exchange_weak(false, true, Ordering::SeqCst, Ordering::Relaxed).is_ok() {
            Ok(SpinLockGuard { lock: self })
        } else {
            Err(())
        }
    }

    pub fn lock(&self) -> SpinLockGuard<T> {
        loop {
            if let Ok(guard) = self.try_lock() {
                return guard;
            }
            hint::spin_loop();
        }
    }
}

impl<'a, T:? Sized> Deref for SpinLockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {&*self.lock.data.get()}
    }
}

impl<'a, T: ?Sized> DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {&mut *self.lock.data.get()}
    }
}

impl<'a, T: ?Sized> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.unlock();
    }
}