use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};
mod trie;
use std::env::current_dir;
use trie::Trie;

fn load_ocd2_from_current_dir(file_name: &str) -> HashMap<String, Vec<String>> {
    let file_path = current_dir().unwrap().join(file_name);
    let file = fs::File::open(file_path).expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut mapping = HashMap::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut parts = line.split_whitespace();
        if let Some(key) = parts.next() {
            let values: Vec<String> = parts.map(|s| s.to_string()).collect();
            mapping.insert(key.to_string(), values);
        }
    }

    mapping
}
pub fn converter(input: &str, tp: &str) -> String {
    let mut chars: HashMap<String, Vec<String>>;
    let mut phrases: HashMap<String, Vec<String>>;
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
