pub mod memory_manager;
pub use memory_manager::MemoryManager;
pub struct OSInterface;

impl OSInterface {
    pub fn request_memory(size: usize) -> Result<*mut u8, String> {
        MemoryManager::alloc_block(size)
    }

    pub fn release_memory(ptr: *mut u8, size: usize) -> Result<(), String> {
        MemoryManager::free_block(ptr, size)
    }
}
