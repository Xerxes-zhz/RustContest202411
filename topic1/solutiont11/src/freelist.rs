use std::ptr;
pub struct FreeList {
    head: *mut Node,
}

struct Node {
    next: *mut Node,
}

impl FreeList {
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, ptr: *mut u8) {
        unsafe {
            let node = ptr as *mut Node;
            (*node).next = self.head;
            self.head = node;
        }
    }

    pub fn pop(&mut self) -> Option<*mut u8> {
        if self.head.is_null() {
            None
        } else {
            unsafe {
                let node = self.head;
                self.head = (*node).next;
                Some(node as *mut u8)
            }
        }
    }
}
