
// use minimalloc::MiniAllocator;
// use std::alloc::{GlobalAlloc, Layout};

// #[test]
// fn test_global_allocator_basic() {
//     let layout = Layout::from_size_align(128, 8).unwrap();
//     unsafe {
//         let ptr = std::alloc::alloc(layout);
//         assert!(!ptr.is_null(), "Failed to allocate memory");

//         std::alloc::dealloc(ptr, layout);
//         // 没有崩溃即表示释放成功
//     }
// }

// #[test]
// fn test_heap_allocation_and_deallocation() {
//     let mut heap = Heap::new(4096); // 初始化 4KB 堆

//     // 分配内存
//     let ptr1 = heap.allocate(128).unwrap();
//     let ptr2 = heap.allocate(256).unwrap();

//     // 确认分配地址不重复
//     assert_ne!(ptr1, ptr2);

//     // 释放内存
//     heap.deallocate(ptr1, 128);
//     assert!(
//         heap.allocate(128).is_some(),
//         "Freed memory should be reusable"
//     );
// }
// #[test]
// fn test_freelist_allocation_and_free() {
//     let mut freelist = FreeList::new();

//     // 添加空闲块
//     freelist.add_free_block(0x1000 as *mut u8, 128);
//     freelist.add_free_block(0x2000 as *mut u8, 256);

//     // 分配块
//     let block = freelist.find_suitable_block(128).unwrap();
//     assert_eq!(block.size, 128);

//     // 释放块
//     freelist.deallocate_block(block);
//     assert!(freelist.find_suitable_block(128).is_some());
// }

// #[test]
// fn test_freelist_merge_blocks() {
//     let mut freelist = FreeList::new();

//     // 添加相邻块
//     freelist.add_free_block(0x1000 as *mut u8, 128);
//     freelist.add_free_block(0x1080 as *mut u8, 128);

//     // 合并相邻块
//     freelist.merge_blocks();
//     assert_eq!(freelist.total_free_size(), 256);
// }
// #[test]
// fn test_block_metadata() {
//     let mut block = BlockMetadata::new(128); // 128 bytes block
//     assert!(block.is_free, "Block should be free initially");

//     block.mark_allocated();
//     assert!(!block.is_free, "Block should be marked as allocated");

//     block.mark_free();
//     assert!(block.is_free, "Block should be marked as free");
// }
// #[test]
// fn test_os_allocate_and_deallocate() {
//     unsafe {
//         let size = 4096; // 4KB
//         let ptr = os_allocate(size);
//         assert!(!ptr.is_null(), "Failed to allocate memory");

//         os_deallocate(ptr, size);
//         // 没有崩溃即表示释放成功。
//     }
// }
