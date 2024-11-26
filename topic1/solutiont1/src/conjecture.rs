use std::cmp::max;
pub fn goldbach_conjecture() -> u64 {
    let mut num: Vec<bool> = vec![false, false, true];
    let mut primes: Vec<usize> = vec![2];
    let mut left_o: usize = 3; // 动态区间左开
    let mut right_c: usize = 4; // 动态区间右闭
    let mut squares2: Vec<usize> = vec![2, 8]; // 平方*2
    let mut result: Vec<u64> = Vec::new();
    let mut sums: Vec<bool> = vec![true; 3];
    // 动态区间素数筛
    loop {
        if right_c >= num.len() {
            num.resize(right_c, true); //动态拓展num组
            sums.resize(right_c, false); //动态拓展求和结果组
        }
        // 素数筛
        for p in primes.iter() {
            let first = max(p * p, ((left_o + p - 1) / p) * p); // left 向上取整和 p^2 中大的
            let _ = num
                .iter_mut()
                .skip(first)
                .step_by(*p)
                .filter(|x| **x)
                .for_each(|x| *x = false);
        }
        // 合数判断, 素数push, 所有数计算平方*2
        // 这个算法是O(N*M*L) 可以优化到O(N*M+L) 即在目标区间内用squares2和prime的和筛奇合数
        let _ = num.iter().enumerate().skip(left_o).for_each(|(i, x)| {
            match x {
                true => primes.push(i), // 素数
                false => {
                    // 合数
                    if i % 2 == 1 //奇
                        && !squares2
                            .iter()
                            .any(|&x| x < i && primes.contains(&(i - x)))
                    // 有可行解
                    {
                        result.push(i as u64);
                    }
                }
            }
            squares2.push(i * i * 2);
        });
        left_o = right_c + 1;
        right_c *= 2;
        if result.len() >= 2 {
            break;
        }
    }
    result.iter().take(2).sum::<u64>()
}
