
mod allocator;
mod freelist;
mod heap;
mod metadata;
mod os;
mod thread;

pub use allocator::MiniAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: MiniAllocator = MiniAllocator;
