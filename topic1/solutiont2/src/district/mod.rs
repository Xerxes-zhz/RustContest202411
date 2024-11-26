mod dsu;
mod province;
use province::Province;

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
pub fn count_batch(batch: Vec<Province>) -> String {
    let mut province_dsu = dsu::DSU::new();
    for mut p in batch.into_iter() {
        p.prepare(); // 初始化成便于并查集规则的集合
        p.into_dsu_iter()
            .for_each(|(root, sub)| province_dsu.init(root, sub));
    }
    province_dsu.count_trees()
}

pub fn count_provinces() -> String {
    let district_json_path = current_dir().unwrap().join("district.json");
    let json = json_read(district_json_path);
    let batches = Province::from_json_using_fsm(json); // 基于有限状态自动机实现的json解析
    batches
        .into_iter()
        .map(|batch| count_batch(batch))
        .reduce(|res, new| format!("{},{}", res, new))
        .expect("合并异常")
}
