use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
static RULES: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"^原法定退休年龄(?P<age>\d{2})周岁(?P<gender>女|男)职工$").unwrap(),
        Regex::new(r"^(?P<gender>女|男)职工$").unwrap(),
    ]
});
#[derive(Deserialize, Debug, Hash, Eq, PartialEq)]
enum Gender {
    Female,
    Male,
}
#[derive(Debug)]
struct RetirementInfo {
    gender: Gender,
    age: Option<i32>,
    year: i32,
    month: i32,
}
impl RetirementInfo {
    fn from(time: &str, tp: &str) -> Option<Self> {
        if let Some((year, month)) = Self::parse_date(time) {
            for rule in RULES.iter() {
                if let Some(caps) = rule.captures(tp) {
                    let gender = match &caps["gender"] {
                        "女" => Gender::Female,
                        "男" => Gender::Male,
                        _ => return None,
                    };
                    let age = caps
                        .name("age")
                        .map(|a| a.as_str().parse::<i32>().ok())
                        .flatten();
                    return Some(RetirementInfo {
                        gender,
                        age,
                        year,
                        month,
                    });
                } else {
                    continue;
                }
            }
            None
        } else {
            None
        }
    }
    fn parse_date(input: &str) -> Option<(i32, i32)> {
        let parts: Vec<&str> = input.split('-').collect();
        if parts.len() != 2 {
            return None; // 确保格式为 "YYYY-MM"
        }

        let year = parts[0].parse::<i32>().ok()?;
        let month = parts[1].parse::<i32>().ok()?;

        if month < 1 || month > 12 {
            return None;
        }
        Some((year, month))
    }
    fn prepare(&mut self, config: &Config) {
        if self.age.is_none() {
            self.age = config.get_default_age(&self.gender)
        }
    }
}
#[derive(Deserialize, Debug)]
struct Rule {
    gender: Gender,
    age: i32,
    delay_per_month: f32,
    max_age: i32,
}
#[derive(Deserialize, Debug)]

struct Config {
    default: HashMap<Gender, Option<i32>>,
    rules: Vec<Rule>,
    start_year: i32,
    start_month: i32,
}
impl Config {
    fn get_default_age(&self, gender: &Gender) -> Option<i32> {
        self.default.get(gender).expect("wrong gender").clone()
    }
    fn calc_left_month(
        &self,
        month_left: i32,
        delay_per_month: f32,
        max_months: i32,
    ) -> (i32, i32) {
        let n = month_left as f32; // 从预计月份开始
        let delay = (n * delay_per_month).ceil() as i32;
        let delay = {
            if month_left + delay > max_months {
                max_months - month_left // 剩余最大延迟
            } else {
                delay
            }
        };
        ((month_left + delay - 1), delay)
    }
    fn calc_retire_time(&self, info: &RetirementInfo) -> Option<String> {
        if let Some(rule) = self
            .rules
            .iter()
            .find(|r| Some(r.age) == info.age && r.gender == info.gender)
        {
            //  不算1月当月
            let mut final_month = self.start_month - 1;
            let mut final_year = self.start_year;
            let mut retire_age: f32 = rule.age as f32;
            let mut delay = 0;
            let mut month_real_left = 0;
            // 25年1月后剩下的月
            let month_left =
                rule.age * 12 - ((final_year - info.year) * 12 + (final_month - info.month));

            if month_left <= 0 {
                final_year = info.year;
                final_month = info.month;
                final_month += rule.age * 12 - 1;
            } else {
                // 每个月延迟d个月, t个月后退休,则t-[t*d]>=month_left时成立
                // 即 t>= month_real_left/(1-d)
                let max_months: i32 = rule.max_age * 12
                    - ((final_year - info.year) * 12 + (final_month - info.month));
                (month_real_left, delay) =
                    self.calc_left_month(month_left, rule.delay_per_month, max_months);
                retire_age = rule.age as f32 + (delay as f32 / 12.0);
                // 增量从1月当月 开始
                final_month += month_real_left;
                // 整除从0开始
            }
            final_year += (final_month - 1) / 12;
            final_month %= 12;
            final_month += 1;
            if retire_age.fract() == 0.0 {
                Some(format!(
                    "{}-{:0>2},{},{}",
                    final_year, final_month, retire_age as i32, delay
                ))
            } else {
                Some(format!(
                    "{}-{:0>2},{:.2},{}",
                    final_year, final_month, retire_age, delay
                ))
            }
        } else {
            None
        }
    }
}
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
