#![allow(unused_variables, dead_code, unused_imports)]
mod allocator;
mod freelist;

pub mod heap;
mod metadata;
pub mod os {
    pub mod memory_manager;
}
mod thread;

pub use allocator::RSMAllocator;

// #[global_allocator]
// static GLOBAL_ALLOCATOR: RSMAllocator = RSMAllocator;
//
//
// 上班族时间有限, 没能完成
// 仅实现了heap和freelist的创建
// 从mmap里获取了大块内存并分区管理
