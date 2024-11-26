use std::{
    hash::{Hash, Hasher},
    u64, usize,
};
// Hasher 用一个素数做因子, 乘每一位的u8, 累加取余得到hash
struct MyHasher {
    hash: u64,
}
impl MyHasher {
    fn new() -> Self {
        MyHasher { hash: 0 }
    }
}
impl Hasher for MyHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            // 乘个素数
            self.hash = self.hash.wrapping_mul(103).wrapping_add(byte as u64);
        }
    }
    fn finish(&self) -> u64 {
        self.hash
    }
}

// 存每个hash下的结果, 自动计数, 插入时返回是否是新插入
struct Bucket<K> {
    entries: Vec<(K, u64)>,
}
impl<K> Bucket<K>
where
    K: Hash + Eq + Clone,
{
    fn insert(&mut self, key: K) -> bool {
        let mut new_insert = true;
        for entry in self.entries.iter_mut() {
            // 插入不需要考虑借用
            if entry.0 == key {
                new_insert = false;
                entry.1 = entry.1 + 1 as u64;
                break;
            }
        }
        if new_insert {
            self.entries.push((key, 1 as u64));
        }
        return new_insert;
    }
}

// 插入时自动计数的HashMap
struct CountHashMap<K> {
    buckets: Vec<Bucket<K>>,
    capacity: u64,
    distinct_count: usize,
}
impl<K> CountHashMap<K>
where
    K: Hash + Eq + Clone,
{
    fn new(capacity: u64) -> CountHashMap<K>
    where
        K: Hash + Eq + Clone,
    {
        // 不能直接初始化vec, 因为hashmap会访问所有index, 所以必须初始化出Bucket
        // let buckets: Vec<Bucket<K>> = Vec::with_capacity(capacity as usize);
        let buckets: Vec<Bucket<K>> = (0..capacity)
            .map(|_| Bucket {
                entries: Vec::new(),
            })
            .collect::<Vec<Bucket<K>>>();
        CountHashMap {
            buckets,
            capacity,
            distinct_count: 0,
        }
    }
    // val是count, 不需要传
    fn insert(&mut self, key: K) {
        let index = self.hash(&key);
        let bucket = &mut self.buckets[index];
        let new_insert = bucket.insert(key);
        if new_insert {
            self.distinct_count += 1;
        }
    }
    fn hash<Q: ?Sized>(&self, key: &Q) -> usize
    where
        Q: Hash,
        K: std::borrow::Borrow<Q>,
    {
        let mut hasher = MyHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % self.capacity as u64) as usize
    }
}
pub fn new_count_distinct(input_str: &str) -> usize {
    let mut hash_map: CountHashMap<String> = CountHashMap::new(10);
    let inputs = input_str.split(",");
    for i in inputs.into_iter() {
        hash_map.insert(i.to_string());
    }
    hash_map.distinct_count
}
