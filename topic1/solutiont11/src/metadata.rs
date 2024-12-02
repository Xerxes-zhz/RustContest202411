pub struct BlockMetadata {
    pub size: usize,
    pub is_free: bool,
}

impl BlockMetadata {
    pub fn new(size: usize) -> Self {
        BlockMetadata { size, is_free: true }
    }
}

