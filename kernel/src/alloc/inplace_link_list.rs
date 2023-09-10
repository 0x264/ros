use core::ptr;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct InplaceLinkList {
    head: *mut Node
}

#[repr(transparent)]
#[derive(Clone, Copy)]
struct Node {
    next: *mut Node
}

impl InplaceLinkList {
    pub const fn new() -> Self {
        Self { head: ptr::null_mut() }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    pub unsafe fn push(&mut self, item: *mut usize) {
        let mut node = item as *mut Node;
        (*node).next = self.head;
        self.head = node;
    }

    pub unsafe fn pop(&mut self) -> Option<*mut usize> {
        if self.is_empty() {
            None
        } else {
            let node = self.head;
            self.head = (*node).next;
            Some(node as *mut usize)
        }
    }

    pub unsafe fn remove(&mut self, item: *mut usize) -> bool {
        if self.is_empty() || item == ptr::null_mut() {
            return false;
        }

        let node = item as *mut Node;
        if self.head == node {
            self.head = (*self.head).next;
            return true;
        }

        let mut p = self.head;
        let mut q = (*p).next;
        while !q.is_null() {
            if q == node {
                (*p).next = (*q).next;
                return true;
            }
            p = q;
            q = (*q).next;
        }

        false
    }
}