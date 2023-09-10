use core::{mem::size_of, ptr};

use super::inplace_link_list::InplaceLinkList;

const MAX_LEVEL: usize = 39;
const MIN_BLOCK_SIZE: usize = size_of::<usize>();
const MAX_ALIGN: usize = 4096;

// align should be power of 2
fn align_up(addr: usize, align: usize) -> usize {
    let v = align - 1;
    (addr + v) & !v
}

// align should be power of 2
fn align_down(addr: usize, align: usize) -> usize {
    addr & !(align - 1)
}

pub struct BuddyAllocator {
    free_list: [InplaceLinkList; MAX_LEVEL + 1],
    start_addr: *mut usize,
    size: usize,
    level: usize
}

unsafe impl Send for BuddyAllocator{}

impl BuddyAllocator {
    pub const fn new() -> Self {
        Self {
            free_list: [InplaceLinkList::new(); MAX_LEVEL + 1],
            start_addr: ptr::null_mut(),
            size: 0,
            level: 0
        }
    }

    pub unsafe fn init(&mut self, start: usize, end: usize) {
        let start = align_up(start, MAX_ALIGN);
        let end = align_down(end, MAX_ALIGN);

        assert!(start < end);

        let size = end - start;
        let size = core::cmp::min(size, MIN_BLOCK_SIZE << MAX_LEVEL);

        let mut level = 1;
        while (MIN_BLOCK_SIZE << level) < size {
            level += 1;
        }

        let start_addr = start as *mut usize;
        self.free_list[0].push(start_addr);
        self.start_addr = start_addr;
        self.size = size;
        self.level = level;
    }

    pub unsafe fn alloc(&mut self, request_size: usize, align: usize) -> *mut usize {
        assert!(align <= MAX_ALIGN);

        if let Some(level) = self.find_level_for_size(request_size) {
            self.get_block_for_level(level)
        } else {
            ptr::null_mut()
        }
    }

    pub unsafe fn dealloc(&mut self, addr: *mut usize, size: usize) {
        if addr == ptr::null_mut() {
            return;
        }

        let level = self.find_level_for_size(size).expect("is this addr alloc by us?");

        if level == 0 {
            assert_eq!(addr, self.start_addr);
        } else {
            let block1 = (self.start_addr as usize) + (self.size >> level);
            let block2 = block1 + 1;
            assert!((addr as usize) == block1 || (addr as usize) == block2);
        }

        self.free_addr(level, addr);
    }

    unsafe fn free_addr(&mut self, level: usize, addr: *mut usize) {
        let list = self.free_list.get_unchecked_mut(level);
        if level == 0 {
            list.push(addr);
        } else {
            let buddy = (addr as usize) ^ 1;
            if list.remove(buddy as *mut usize) {
                self.free_addr(level - 1, ((addr as usize) >> 1) as *mut usize);
            } else {
                list.push(addr);
            }
        }
    }

    unsafe fn get_block_for_level(&mut self, level: usize) -> *mut usize {
        let addr = self.free_list.get_unchecked_mut(level).pop();
        if level == 0 || addr != ptr::null_mut() {
            addr
        } else {
            let parent_block_addr = self.get_block_for_level(level - 1);
            if parent_block_addr != ptr::null_mut() {
                let addr = (parent_block_addr as usize) << 1;
                self.free_list.get_unchecked_mut(level).push((addr + 1) as *mut usize);
                addr as *mut usize
            } else {
                ptr::null_mut()
            }
        }
    }

    fn find_level_for_size(&self, request_size: usize) -> Option<usize> {
        if request_size > self.size {
            None
        } else {
            let mut level = 0;
            while (self.size >> level) > request_size {
                level += 1;
            }
            Some(core::cmp::min(self.level, level))
        }
    }
}