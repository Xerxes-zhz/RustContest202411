#![cfg_attr(test, allow(dead_code, unused_variables))]
mod allocator;
mod freelist;
mod heap;
mod metadata;
pub mod os {
    pub mod memory_manager;
}
mod thread;

pub use allocator::RSMAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: RSMAllocator = RSMAllocator;
