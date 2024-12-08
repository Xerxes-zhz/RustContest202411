use std::env::current_dir;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
mod retirement_info;
use retirement_info::RetirementInfo;
mod config;
use config::Config;
use once_cell::sync::Lazy;
use regex::Regex;
static RULES: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"^原法定退休年龄(?P<age>\d{2})周岁(?P<gender>女|男)职工$").unwrap(),
        Regex::new(r"^(?P<gender>女|男)职工$").unwrap(),
    ]
});
fn json_read(path: PathBuf) -> String {
    let mut file = File::open(path).expect("can't open file");
    let mut json_str = String::new();
    let _ = file.read_to_string(&mut json_str);
    json_str
}
pub fn retire_time(time: &str, tp: &str) -> String {
    // 解析部分使用正则而非字符串解析是出于后续改动尽可能小的题目要求
    // 正则表达式更符合通常意义的生产需求
    // 同样的原因用once_cell动态编译了正则规则
    // 采用规则解析以备变动
    // 规则配置在json中, 可拓展Gender以支持不同性别, 因认为无必要未为支持
    let rules_path = current_dir().unwrap().join("rules.json");
    let rules_json = json_read(rules_path);
    let config: Config = serde_json::from_str(&rules_json).expect("parse failed");
    if let Some(mut retirement_info) = RetirementInfo::from(time, tp) {
        retirement_info.prepare(&config);
        config
            .calc_retire_time(&retirement_info)
            .expect("未配置规则")
    } else {
        panic!("信息规则错误, 需要调整适应新规则");
    }
}
