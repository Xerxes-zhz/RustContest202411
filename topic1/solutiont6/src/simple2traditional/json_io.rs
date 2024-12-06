use serde::Deserialize;
use std::env::current_dir;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
fn json_read(path: PathBuf) -> String {
    let mut file = File::open(path).expect("can't open file");
    let mut json_str = String::new();
    let _ = file.read_to_string(&mut json_str);
    json_str
}
pub fn json_parse_from_current_dir<T>(json_name: &str) -> T
where
    T: for<'de> Deserialize<'de>,
{
    let rules_path = current_dir().unwrap().join(json_name);
    let rules_json = json_read(rules_path);
    serde_json::from_str::<T>(&rules_json).expect("parse failed")
}
