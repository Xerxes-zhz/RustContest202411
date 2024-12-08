use rsmalloc::os::memory_manager::MemoryManager;

#[test]
fn test_alloc_large_block() {
    let size = 1024 * 1024; // 1 MB
    match MemoryManager::alloc_block(size) {
        Ok(ptr) => {
            assert!(!ptr.is_null(), "Memory pointer should not be null");
            println!("Memory allocated at: {:?}", ptr);

            // Free the memory to ensure it works as expected
            let result = MemoryManager::free_block(ptr, size);
            assert!(result.is_ok(), "Memory should be freed successfully");
        }
        Err(e) => panic!("Memory allocation failed: {}", e),
    }
}

#[test]
fn test_free_invalid_memory() {
    // Freeing an invalid pointer should fail
    let invalid_ptr = 0x12345678 as *mut u8;
    let size = 1024 * 1024;

    let result = MemoryManager::free_block(invalid_ptr, size);
    assert!(result.is_err(), "Freeing invalid memory should fail");
}
