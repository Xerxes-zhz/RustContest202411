// use rand::Rng;
use std::cmp::min;
fn prime_sieve(limit: usize) -> Vec<u128> {
    let mut sieve = vec![true; limit + 1];
    let primes: Vec<usize> = vec![2];
    for p in primes.iter() {
        let _ = sieve
            .iter_mut()
            .skip(p * p)
            .step_by(*p)
            .for_each(|x| *x = false);
    }
    (2..=limit)
        .filter(|&x| sieve[x])
        .map(|x| x as u128)
        .collect()
}
fn trial_division(n: u128, primes: &Vec<u128>, only_sieve: bool) -> Option<u128> {
    if n < 2 {
        return None;
    }

    let prime_max = *primes.last().unwrap();
    for i in primes.into_iter() {
        if n % i == 0 {
            return Some(*i); // 找到一个因子
        }
    }
    if !only_sieve {
        for i in (prime_max..=((n as f64).sqrt() as u128)).step_by(2) {
            if n % i == 0 {
                return Some(i); // 找到一个因子
            }
        }
    }

    None // 没有找到因子，说明是素数
}

// /// 计算 (a * b) % m，防止溢出的乘法模运算
// fn mod_mul(a: u128, b: u128, m: u128) -> u128 {
//     let mut result = 0;
//     let mut x = a % m;
//     let mut y = b % m;

//     while y > 0 {
//         if y % 2 == 1 {
//             result = (result + x) % m;
//         }
//         x = (2 * x) % m;
//         y /= 2;
//     }
//     result
// }

// /// 计算 (base ^ exp) % m 的幂模运算
// fn mod_pow(base: u128, exp: u128, m: u128) -> u128 {
//     let mut result = 1;
//     let mut base = base % m;
//     let mut exp = exp;

//     while exp > 0 {
//         if exp % 2 == 1 {
//             result = mod_mul(result, base, m);
//         }
//         base = mod_mul(base, base, m);
//         exp /= 2;
//     }
//     result
// }

// fn gcd(mut a: u128, mut b: u128) -> u128 {
//     while b != 0 {
//         let temp = b;
//         b = a % b;
//         a = temp;
//     }
//     a
// }
// /// Pollard Rho 算法，寻找 n 的一个非平凡因子
// fn pollard_rho(n: u128) -> Option<u128> {
//     let mut rng = rand::thread_rng();
//     let mut x: u128 = rng.gen_range(2..n);
//     let mut y = x;
//     let mut c: u128 = rng.gen_range(1..n);
//     let mut d: u128 = 1;

//     while d == 1 {
//         x = (mod_mul(x, x, n) + c) % n;
//         y = (mod_mul(y, y, n) + c) % n;
//         y = (mod_mul(y, y, n) + c) % n;
//         d = gcd((x as i128 - y as i128).abs() as u128, n);
//     }

//     if d != 1 {
//         return Some(d);
//     }

//     None
// }

// fn ecm_factorize(n: u128) -> Option<u128> {
//     todo!()
// }
pub fn find_max_prime_factor(number: u128) -> u128 {
    const TEN_12: u128 = 10_u128.pow(12);
    // const TEN_20: u128 = 10_u128.pow(20);
    const TEN_6: u128 = 10_u128.pow(6);
    const TEN_9: u128 = 10_u128.pow(9);
    // u128的范围约在 10^38
    let mut factors = Vec::new();
    let mut n = number;
    let small_primes = prime_sieve(min(TEN_6 as usize, ((number + 1) as f64).sqrt() as usize));
    // 先用小素数筛一遍
    while n > 1 {
        if let Some(factor) = trial_division(n, &small_primes, true) {
            factors.push(factor);
            n /= factor;
        } else {
            break;
        }
    }
    // 正式计算
    while n > 1 {
        match n {
            0..TEN_12 => {
                // 小，试除法
                if let Some(factor) = trial_division(n, &small_primes, false) {
                    factors.push(factor);
                    n /= factor;
                } else {
                    break;
                }
            }
            _ => {
                break;
            } // TEN_12..=TEN_20 => {
              //     // 中等，Pollard-Rho
              //     if let Some(factor) = pollard_rho(n) {
              //         factors.push(factor);
              //         n /= factor;
              //     } else {
              //         break;
              //     }
              // }
              // _ => {
              //     break;
              //     // 大，使用 ECM
              //     if let Some(factor) = ecm_factorize(n) {
              //         factors.push(factor);
              //         n /= factor;
              //     } else {
              //         break;
              //     }
              // }
        }
    }

    if n > 1 {
        factors.push(n);
    }

    factors.sort();
    *factors.last().unwrap()
}
