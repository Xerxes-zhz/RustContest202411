use crate::os::memory_manager::MemoryManager;

pub struct Region {
    pub start: *mut u8,
    pub size: usize,
    pub in_use: bool,
}

pub struct RegionManager {
    regions: Vec<Region>,
}

impl RegionManager {
    pub fn new() -> Self {
        Self {
            regions: Vec::new(),
        }
    }

    pub fn allocate_region(&mut self, size: usize) -> Result<&Region, String> {
        match MemoryManager::alloc_block(size) {
            Ok(ptr) => {
                self.regions.push(Region {
                    start: ptr,
                    size,
                    in_use: true,
                });
                Ok(self.regions.last().unwrap())
            }
            Err(e) => Err(e),
        }
    }

    pub fn free_region(&mut self, ptr: *mut u8) -> Result<(), String> {
        if let Some(index) = self.regions.iter().position(|r| r.start == ptr) {
            let region = self.regions.remove(index);
            MemoryManager::free_block(region.start, region.size)
        } else {
            Err("Region not found".to_string())
        }
    }
}
