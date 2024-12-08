pub fn prime_sieve(limit: usize) -> Vec<u128> {
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
