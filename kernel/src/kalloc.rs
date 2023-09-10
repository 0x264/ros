use core::alloc::{GlobalAlloc, Layout};

use crate::{spinlock::SpinLock, alloc::buddy_allocator::BuddyAllocator};

#[global_allocator]
pub static KALLOC: KAlloc = KAlloc(SpinLock::new(BuddyAllocator::new()));

pub struct KAlloc(SpinLock<BuddyAllocator>);

unsafe impl GlobalAlloc for KAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match layout.size() {
            0 => layout.dangling().as_ptr(),
            size => self.0.lock().alloc(size, layout.align()) as *mut u8
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        match layout.size() {
            0 => (),
            size => self.0.lock().dealloc(ptr as *mut usize, size)
        }
    }
}

pub unsafe fn init(start: usize, end: usize) {
    KALLOC.0.lock().init(start, end);
}