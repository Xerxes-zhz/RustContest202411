pub fn new_birthday_probability(n: u32) -> f64 {
    if n > 365 {
        return 1.000 as f64;
    } else if n <= 1 {
        return 0.000 as f64;
    } else {
        let p_distinct = (1..=n)
            .map(|i| (365.0 - i as f64 + 1.0) / (365.0))
            .reduce(|acc, e| acc * e)
            .unwrap();
        ((1.0 - p_distinct) * 10000.0).round() / 10000.0
    }
}
