use std::collections::HashMap;
mod ocd2;
mod trie;
use ocd2::load_ocd2_from_current_dir;
use trie::Trie;

pub fn converter(input: &str, tp: &str) -> String {
    // 使用和pinyin相同的前缀树和分词
    // 分词采用贪婪匹配
    // 数据来自多个数据合并后的ocd2数据

    let chars: HashMap<String, Vec<String>>;
    let phrases: HashMap<String, Vec<String>>;
    match tp {
        "t2s" => {
            chars = load_ocd2_from_current_dir("TSCharacters.txt");
            phrases = load_ocd2_from_current_dir("TSPhrases.txt");
        }
        "s2t" => {
            chars = load_ocd2_from_current_dir("STCharacters.txt");
            phrases = load_ocd2_from_current_dir("STPhrases.txt");
        }
        _ => {
            unreachable!()
        }
    }
    let mut trie = Trie::new();
    for (k, v) in chars {
        trie.insert(&k, v);
    }
    for (k, v) in phrases {
        trie.insert(&k, v);
    }
    let mut flat_word = Vec::new();
    let segments = trie.segment(input);
    for word in segments {
        if let Some(ch) = trie.search(&word) {
            flat_word.extend(ch);
        } else {
            flat_word.push(word.clone());
        }
    }
    flat_word.join("")
}
