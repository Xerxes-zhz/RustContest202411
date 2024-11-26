use regex::Regex;
use std::collections::HashMap;
// from base n to base m
fn convert_base_n2m(num: String, n: u32, m: u32) -> String {
    // 构造数位
    let digits = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f",
    ];
    let mut char_to_num_map = HashMap::new();
    let mut num_to_char_map = HashMap::new();
    for (i, &ch) in digits.iter().enumerate() {
        char_to_num_map.insert(ch, i as u32);
        num_to_char_map.insert(i as u32, ch);
    }

    // 转10进制
    let mut num_base_10 = 0;
    let mut factor = 1;
    for ch in num.chars().rev() {
        let ch: &str = &ch.to_string();
        num_base_10 += factor * char_to_num_map.get(&ch).unwrap();
        factor *= n;
    }

    // 最大因子
    let mut factor = 1;
    loop {
        if factor * m > num_base_10 {
            break;
        } else {
            factor *= m;
        }
    }

    //逐位计数
    let mut output_num = String::new();
    loop {
        let digit_num = num_base_10 / factor;
        let digit_str = num_to_char_map.get(&(digit_num)).unwrap();
        output_num.push_str(digit_str);
        num_base_10 %= factor;
        if factor == 1 {
            break;
        }
        factor /= m;
    }

    output_num
}
pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let re = Regex::new(r"(\d+)\((\d+)\)").unwrap();
    let captures = re.captures(num_str).unwrap();
    let input_num: String = captures[1].to_string();
    let input_base: u32 = captures[2].parse().expect("Failed to parse base");
    convert_base_n2m(input_num, input_base, to_base)
}
