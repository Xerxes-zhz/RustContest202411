pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    // 边缘情况
    if threshold == 0 || threshold == 1 {
        return 0 as u32;
    }
    if threshold == 2 {
        return 2 as u32;
    }

    // init
    let mut a = 1;
    let mut b = 1;
    let mut sum: u32 = 2;

    loop {
        b = a + b;
        a = b - a;
        if b >= threshold {
            break;
        } else if b % 2 == 1 {
            sum += b;
        }
    }
    sum
}
