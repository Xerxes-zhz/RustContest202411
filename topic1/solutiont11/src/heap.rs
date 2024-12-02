use std::alloc::Layout;

pub fn allocate(layout: Layout) -> *mut u8 {
    // 分配逻辑：从空闲链表中查找或扩展堆
    crate::freelist::allocate(layout.size())
}

pub fn deallocate(ptr: *mut u8, layout: Layout) {
    // 回收逻辑：将内存块插入自由链表
    crate::freelist::deallocate(ptr, layout.size());
}

