use serde::Deserialize;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
mod trie;
use trie::Trie;
fn remove_tone_numbers(pinyin_with_tone: &str) -> String {
    // 定义数字字符，用于去掉拼音中的音调数字
    pinyin_with_tone
        .chars()
        .filter(|ch| !ch.is_ascii_digit()) // 过滤掉 ASCII 数字
        .collect()
}
// 定义 JSON 数据对应的结构体
#[derive(Debug, Deserialize)]
struct PinyinData {
    // 动态键值对，键是中文词语，值是拼音列表
    #[serde(flatten)]
    entries: HashMap<String, Vec<String>>,
}

fn capitalize_first_letter(input: &str) -> String {
    let mut chars = input.chars(); // 将字符串转为字符迭代器
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(), // 如果输入为空，返回空字符串
    }
}

fn json_read(path: PathBuf) -> String {
    let mut file = File::open(path).expect("can't open file");
    let mut json_str = String::new();
    let _ = file.read_to_string(&mut json_str);
    json_str
}
pub fn converter(input: &str) -> String {
    //path
    let phrases_path = current_dir().unwrap().join("phrases.json");
    let tone_path = current_dir().unwrap().join("tone.json");
    let wade_giles_path = current_dir().unwrap().join("wade_giles.json");
    let pinyin_path = current_dir().unwrap().join("pinyin.json");

    // json
    let phrases_json = json_read(phrases_path);
    let tone_json = json_read(tone_path);
    let wade_giles_json = json_read(wade_giles_path);
    let pinyin_json = json_read(pinyin_path);

    let pinyin_map: HashMap<u32, String> =
        serde_json::from_str(&pinyin_json).expect("parse json failed");
    let word_pinyin_map: HashMap<u32, String> = pinyin_map
        .into_iter()
        .map(|(unicode, pinyin)| (unicode, pinyin.split(",").nth(0).map(String::from).unwrap()))
        .collect();
    let tone_map: HashMap<String, String> =
        serde_json::from_str(&tone_json).expect("parse json failed");
    let data: PinyinData = serde_json::from_str(&phrases_json).expect("parse json failed");
    let wade_giles_map: HashMap<String, String> =
        serde_json::from_str(&wade_giles_json).expect("parse json failed");
    let no_tone_list: Vec<(String, String)> = tone_map
        .into_iter()
        .map(|(pinyin, pinyin_with_tone)| (pinyin, remove_tone_numbers(&pinyin_with_tone)))
        .collect();
    let mut trie = Trie::new();
    for (word, pinyin) in data.entries {
        trie.insert(&word, pinyin);
    }
    let segments = trie.segment(input);
    let mut flat_pinyin = Vec::new();

    for word in segments {
        if let Some(pinyin) = trie.search(&word) {
            flat_pinyin.extend(pinyin);
        } else {
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
