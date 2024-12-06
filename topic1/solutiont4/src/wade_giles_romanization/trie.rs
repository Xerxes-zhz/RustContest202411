// 定义前缀树节点
use std::collections::HashMap;
#[derive(Debug, Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
    item: Option<Vec<String>>, // pinyin & 繁简转换
    word: Option<String>,
}

// 定义前缀树
#[derive(Debug, Default)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    // 创建新的 Trie
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    // 插入一个中文词和对应的拼音
    pub fn insert(&mut self, word: &str, item: Vec<String>) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::default);
        }
        node.is_end_of_word = true;
        node.item = Some(item);
        node.word = Some(word.to_string());
    }

    // 查找一个中文词，返回对应的拼音（如果存在）
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
    // UTF-8 分词
    pub fn segment(&self, text: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut i = 0;
        let chars: Vec<char> = text.chars().collect(); // 将字符串按字符分解

        while i < chars.len() {
            let mut node = &self.root;
            let mut longest_word = None;
            let mut last_match_index = i;

            // 从当前位置尝试匹配最长的前缀
            for j in i..chars.len() {
                if let Some(next_node) = node.children.get(&chars[j]) {
                    node = next_node;
                    if node.is_end_of_word {
                        longest_word = node.word.clone();
                        last_match_index = j + 1; // 更新最后匹配的位置
                    }
                } else {
                    break;
                }
            }

            // 如果找到最长匹配，添加到结果
            if let Some(word) = longest_word {
                result.push(word);
                i = last_match_index; // 跳到最后匹配位置
            } else {
                // 如果没有匹配，添加当前字符作为单词
                result.push(chars[i].to_string());
                i += 1;
            }
        }

        result
    }
}
