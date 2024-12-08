use std::ptr;
use libc::{mmap, munmap, MAP_ANONYMOUS, MAP_PRIVATE, PROT_READ, PROT_WRITE};

pub struct MemoryManager;

impl MemoryManager {
    /// Allocates a large memory block using mmap
    pub fn alloc_block(size: usize) -> Result<*mut u8, String> {
        unsafe {
            let ptr = mmap(
                ptr::null_mut(),
                size,
                PROT_READ | PROT_WRITE,
                MAP_PRIVATE | MAP_ANONYMOUS,
                -1,
                0,
            );

            if ptr == libc::MAP_FAILED {
                Err("Failed to allocate memory".to_string())
            } else {
                Ok(ptr as *mut u8)
            }
        }
    }

    /// Frees a memory block using munmap
    pub fn free_block(ptr: *mut u8, size: usize) -> Result<(), String> {
        unsafe {
            if munmap(ptr as *mut libc::c_void, size) == 0 {
                Ok(())
            } else {
                Err("Failed to free memory".to_string())
            }
        }
    }
}

