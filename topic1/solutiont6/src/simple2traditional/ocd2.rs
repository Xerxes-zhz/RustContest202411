use std::env::current_dir;
use std::fs;
use std::io::{self, BufRead};
use std::collections::HashMap;
pub fn load_ocd2_from_current_dir(file_name: &str) -> HashMap<String, Vec<String>> {
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
