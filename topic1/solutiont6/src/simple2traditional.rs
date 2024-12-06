use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};
mod trie;
use trie::Trie;

/// 加载 `ocd2` 数据文件
fn load_ocd2(file_path: &str) -> HashMap<String, Vec<String>> {
    let file = fs::File::open(file_path).expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut mapping = HashMap::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        // 分隔行，第一部分是键，剩余部分是值
        let mut parts = line.split_whitespace();
        if let Some(key) = parts.next() {
            let values: Vec<String> = parts.map(|s| s.to_string()).collect();
            mapping.insert(key.to_string(), values);
        }
    }

    mapping
}
pub fn converter(input: &str, tp: &str) -> String {
    let mut result = String::new();

    for c in text.chars() {
        if let Some(mapped_values) = mapping.get(&c.to_string()) {
            // 默认选择第一个值作为转换结果
            result.push_str(&mapped_values[0]);
        } else {
            // 未匹配到映射，保留原字符
            result.push(c);
        }
    }

    result
}
