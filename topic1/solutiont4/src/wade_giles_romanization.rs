use std::collections::HashMap;
mod json_io;
use json_io::json_parse_from_current_dir;
mod trie;
use trie::Trie;
fn remove_tone_numbers(pinyin_with_tone: &str) -> String {
    pinyin_with_tone
        .chars()
        .filter(|ch| !ch.is_ascii_digit())
        .collect()
}

fn capitalize_first_letter(input: &str) -> String {
    let mut chars = input.chars(); // 将字符串转为字符迭代器
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(), // 如果输入为空，返回空字符串
    }
}

pub fn converter(input: &str) -> String {
    // 基于前缀树实现分词
    // 
    let pinyin_map: HashMap<u32, String> = json_parse_from_current_dir("pinyin.json");
    let tone_map: HashMap<String, String> = json_parse_from_current_dir("tone.json");
    let phrases_data: HashMap<String, Vec<String>> = json_parse_from_current_dir("phrases.json");
    let wade_giles_map: HashMap<String, String> = json_parse_from_current_dir("wade_giles.json");

    // 单字
    let word_pinyin_map: HashMap<u32, String> = pinyin_map
        .into_iter()
        .map(|(unicode, pinyin)| (unicode, pinyin.split(",").nth(0).map(String::from).unwrap()))
        .collect();
    // 去掉声调
    let no_tone_list: Vec<(String, String)> = tone_map
        .into_iter()
        .map(|(pinyin, pinyin_with_tone)| (pinyin, remove_tone_numbers(&pinyin_with_tone)))
        .collect();
    // 前缀树
    let mut trie = Trie::new();
    // 词
    for (word, pinyin) in phrases_data {
        trie.insert(&word, pinyin);
    }
    // 分词
    let segments = trie.segment(input);
    let mut flat_pinyin = Vec::new();
    for word in segments {
        if let Some(pinyin) = trie.search(&word) {
            // 按分词找到词映射词
            flat_pinyin.extend(pinyin);
        } else {
            // 找不到就直接映射字
            for ch in word.chars() {
                let ch: u32 = ch as u32;
                if let Some(p) = word_pinyin_map.get(&ch) {
                    flat_pinyin.push(p.clone());
                }
            }
        }
    }
    let no_tone_pinyin = flat_pinyin
        .iter()
        .map(|pinyin| {
            let mut pinyin = pinyin.clone();
            for (k, v) in no_tone_list.iter() {
                pinyin = pinyin.replace(k, v);
            }
            pinyin
        })
        .collect::<Vec<String>>();
    capitalize_first_letter(
        &no_tone_pinyin
            .iter()
            .map(|pinyin| wade_giles_map.get(pinyin).unwrap().clone())
            .collect::<Vec<String>>()
            .join(" "),
    )
}
