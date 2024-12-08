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
pub fn is_prime_miller_rabin(n: u128, base: [u128; 7]) -> bool {
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
