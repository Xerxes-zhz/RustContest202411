#[test]
fn test_thread_allocator() {
    std::thread::spawn(|| {
        let ptr = crate::thread::thread_allocate(64);
        assert!(!ptr.is_null());
    })
    .join()
    .unwrap();
}
