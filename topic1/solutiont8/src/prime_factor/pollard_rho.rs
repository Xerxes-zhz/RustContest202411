use rand::Rng;
/// 计算 (a * b) % m，防止溢出的乘法模运算
fn mod_mul(a: u128, b: u128, m: u128) -> u128 {
    let mut result = 0;
    let mut x = a % m;
    let mut y = b % m;

    while y > 0 {
        if y % 2 == 1 {
            result = (result + x) % m;
        }
        x = (2 * x) % m;
        y /= 2;
    }
    result
}

/// 计算 (base ^ exp) % m 的幂模运算
fn mod_pow(base: u128, exp: u128, m: u128) -> u128 {
    let mut result = 1;
    let mut base = base % m;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = mod_mul(result, base, m);
        }
        base = mod_mul(base, base, m);
        exp /= 2;
    }
    result
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
/// Pollard Rho 算法，寻找 n 的一个非平凡因子
pub fn pollard_rho(n: u128) -> Option<u128> {
    let mut rng = rand::thread_rng();
    let mut x: u128 = rng.gen_range(2..n);
    let mut y = x;
    let mut c: u128 = rng.gen_range(1..n);
    let mut d: u128 = 1;

    while d == 1 {
        x = (mod_mul(x, x, n) + c) % n;
        y = (mod_mul(y, y, n) + c) % n;
        y = (mod_mul(y, y, n) + c) % n;
        d = gcd((x as i128 - y as i128).abs() as u128, n);
    }

    if d != 1 {
        return Some(d);
    }

    None
}
