use std::collections::HashMap;
#[derive(Debug, Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
    item: Option<Vec<String>>, // pinyin & 繁简转换
    word: Option<String>,
}

#[derive(Debug, Default)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, word: &str, item: Vec<String>) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::default);
        }
        node.is_end_of_word = true;
        node.item = Some(item);
        node.word = Some(word.to_string());
    }

    pub fn search(&self, word: &str) -> Option<Vec<String>> {
        let mut node = &self.root;
        for ch in word.chars() {
            if let Some(next_node) = node.children.get(&ch) {
                node = next_node;
            } else {
                return None;
            }
        }
        if node.is_end_of_word {
            node.item.clone()
        } else {
            None
        }
    }
    pub fn segment(&self, text: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut i = 0;
        let chars: Vec<char> = text.chars().collect();

        while i < chars.len() {
            let mut node = &self.root;
            let mut longest_word = None;
            let mut last_match_index = i;

            for j in i..chars.len() {
                if let Some(next_node) = node.children.get(&chars[j]) {
                    node = next_node;
                    if node.is_end_of_word {
                        longest_word = node.word.clone();
                        last_match_index = j + 1;
                    }
                } else {
                    break;
                }
            }

            if let Some(word) = longest_word {
                result.push(word);
                i = last_match_index;  
            } else {
                result.push(chars[i].to_string());
                i += 1;
            }
        }
        result
    }
}
