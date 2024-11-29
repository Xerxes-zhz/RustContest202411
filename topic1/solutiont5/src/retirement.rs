use once_cell::sync::Lazy;
use regex::Regex;
static RULES: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"^原法定退休年龄(?P<age>\d{2})周岁(?P<gender>女|男)职工$").unwrap(),
        Regex::new(r"^(?P<gender>女|男)职工$").unwrap(),
    ]
});
#[derive(Debug)]
enum Gender {
    Female,
    Male,
}
#[derive(Debug)]
struct RetirementInfo {
    gender: Gender,
    age: Option<u32>,
    year: u32,
    month: u32,
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
                        .map(|a| a.as_str().parse::<u32>().ok())
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
    fn parse_date(input: &str) -> Option<(u32, u32)> {
        // 拆分字符串
        let parts: Vec<&str> = input.split('-').collect();
        if parts.len() != 2 {
            return None; // 确保格式为 "YYYY-MM"
        }

        // 解析年份和月份
        let year = parts[0].parse::<u32>().ok()?; // 解析 year
        let month = parts[1].parse::<u32>().ok()?; // 解析 month

        // 验证月份范围
        if month < 1 || month > 12 {
            return None;
        }

        Some((year, month))
    }
}
struct RetirementRule {
    female_default_age: u32,
    male_default_age: u32,
    
}
pub fn retire_time(time: &str, tp: &str) -> String {
    // 解析部分使用正则而非字符串解析是出于后续改动尽可能小的题目要求
    // 正则表达式更符合通常意义的生产需求
    // 同样的原因用once_cell动态编译了正则规则

    if let Some(retirement_info) = RetirementInfo::from(time, tp) {
        println!("{:?}", retirement_info);
    } else {
        panic!("信息规则错误, 需要调整适应新规则");
    };
    "".to_string()
}
