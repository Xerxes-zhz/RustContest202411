fn decompose(n: u128) -> (u128, u128) {
    let mut d = n - 1;
    let mut s = 0;
    loop {
        if d % 2 == 0 {
            d /= 2;
            s += 1;
        } else {
            break;
        }
    }
    (d, s)
}
fn pow(x: u128, y: u128, n: u128) -> u128 {
    let mut result = 1;
    let mut base = x % n;
    let mut exp = y;

    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % n;
        }
        base = base * base % n;
        exp /= 2;
    }

    result
}
fn is_prime_miller_rabin(n: u128, base: [u128; 7]) -> bool {
    if base.contains(&n) {
        return true;
    }
    let (d, s) = decompose(n);
    'base: for a in base {
        let mut x = pow(a, d, n);
        if (x == 1) || (x == n - 1) {
            continue;
        } else {
            for _ in 0..s {
                x = x * x % n;
                if x == n - 1 {
                    continue 'base;
                }
            }
            return false;
        }
    }
    true
}
pub fn min_edge_prime_num(number: u32) -> String {
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
