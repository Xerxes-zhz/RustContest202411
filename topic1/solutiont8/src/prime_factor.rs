use std::cmp::min;
mod ecm;
mod pollard_rho;
mod prime;
mod prime_sieve;
mod trial_division;

use ecm::ecm_factorize;
use pollard_rho::pollard_rho;
use prime::is_prime_miller_rabin;
use prime_sieve::prime_sieve;
use trial_division::trial_division;

pub fn find_max_prime_factor(number: u128) -> u128 {
    // 测试了超出数据集的数据但超出了时间限制
    // 所以没有实现更大级别的算法
    // 迭代计算, 每找出一个因子就尝试使用更小级别的算法
    // 对不同级别的数据采取不同方案
    // 非常小数据素数筛, 更大的数据也先初始化10^6的素数
    // 在素数集中使用试除法
    // 更大的数据使用 pollard_rho 算法
    // 更大的数据使用ecm椭圆曲线算法, 已经看懂了算法, 但因时间问题并未实现
    const TEN_18: u128 = 10_u128.pow(18);
    const TEN_25: u128 = 10_u128.pow(25);
    const TEN_6: u128 = 10_u128.pow(6);
    const BASE: [u128; 7] = [2, 3, 5, 7, 11, 13, 17];
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
        if is_prime_miller_rabin(n, BASE) {
            break;
        }
        match n {
            0..TEN_18 => {
                // 小，试除法
                if let Some(factor) = trial_division(n, &small_primes, false) {
                    factors.push(factor);
                    n /= factor;
                } else {
                    break;
                }
            }
            TEN_18..=TEN_25 => {
                // 中等，Pollard-Rho
                if let Some(factor) = pollard_rho(n) {
                    factors.push(factor);
                    n /= factor;
                } else {
                    break;
                }
            }
            _ => {
                // 大，使用 ECM
                if let Some(factor) = ecm_factorize(n) {
                    factors.push(factor);
                    n /= factor;
                } else {
                    break;
                }
            }
        }
    }

    if n > 1 {
        factors.push(n);
    }

    factors.sort();
    *factors.last().unwrap()
}
