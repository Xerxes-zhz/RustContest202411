mod prime;
use prime::is_prime_miller_rabin;
pub fn min_edge_prime_num(number: u32) -> String {
    // 使用miller_rabin算法
    // 使用10^12 次方范围100%正确的基数
    let number: usize = number as usize;
    let mut l: u128 = 3;
    let mut all: usize = 5;
    let mut prime_count: usize = 3;
    const BASE: [u128; 7] = [2, 3, 5, 7, 11, 13, 17]; //10^12 100%准确
    match number {
        61..=100 => "3,3".to_string(),
        _ => loop {
            l += 2;
            all += 4;
            prime_count += (1..=3)
                .map(|k| l * l - k * l + k)
                .filter(|p| is_prime_miller_rabin(*p, BASE))
                .count();
            if prime_count * 100 < number * all {
                return format!("{},{}", l, prime_count);
            }
        },
    }
}
