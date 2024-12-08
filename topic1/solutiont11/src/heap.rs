use crate::freelist::FreeList;

mod region_manager;
use region_manager::{Region, RegionManager};
pub struct Heap {
    freelist: FreeList,
    region_manager: RegionManager,
    block_size: usize,
}

impl Heap {
    pub fn new(block_size: usize) -> Self {
        Self {
            freelist: FreeList::new(),
            region_manager: RegionManager::new(),
            block_size,
        }
    }

    /// 分配内存
    pub fn allocate(&mut self) -> Option<*mut u8> {
        // 从空闲列表中分配
        if let Some(ptr) = self.freelist.pop() {
            return Some(ptr);
        }

        // 如果没有空闲块，尝试获取一个新区域
        let region_size = self.block_size * 1024; // 假设区域大小
        match self.region_manager.allocate_region(region_size) {
            Ok(region) => {
                // 将新区域分块并加入空闲列表
                for i in 0..(region.size / self.block_size) {
                    let block_ptr = unsafe { region.start.add(i * self.block_size) };
                    self.freelist.push(block_ptr);
                }

                // 返回一个块
                self.freelist.pop()
            }
            Err(_) => None,
        }
    }

    /// 释放内存
    pub fn free(&mut self, ptr: *mut u8) {
        self.freelist.push(ptr);
    }
}

