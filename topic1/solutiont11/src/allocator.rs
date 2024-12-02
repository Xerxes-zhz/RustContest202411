use std::alloc::{GlobalAlloc, Layout};
use std::ptr;

pub struct MiniAllocator;

unsafe impl GlobalAlloc for MiniAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // 调用自定义堆管理器分配内存
        crate::heap::allocate(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // 调用自定义堆管理器释放内存
        crate::heap::deallocate(ptr, layout)
    }
}

