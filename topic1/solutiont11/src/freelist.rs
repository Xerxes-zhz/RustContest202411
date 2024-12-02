use std::ptr;

pub fn allocate(size: usize) -> *mut u8 {
    // 从自由链表中查找适合的块
    // 如果没有足够的块，则返回空指针
    ptr::null_mut()
}

pub fn deallocate(ptr: *mut u8, size: usize) {
    // 将块标记为空闲并插入自由链表
}

