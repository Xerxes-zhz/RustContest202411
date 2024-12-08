
pub fn trial_division(n: u128, primes: &Vec<u128>, only_sieve: bool) -> Option<u128> {
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
