use super::config::Config;
use super::RULES;
use serde::Deserialize;
#[derive(Deserialize, Debug, Hash, Eq, PartialEq)]
pub enum Gender {
    Female,
    Male,
}
#[derive(Debug)]
pub struct RetirementInfo {
    pub gender: Gender,
    pub age: Option<i32>,
    pub year: i32,
    pub month: i32,
}
impl RetirementInfo {
    pub fn from(time: &str, tp: &str) -> Option<Self> {
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
    pub fn prepare(&mut self, config: &Config) {
        if self.age.is_none() {
            self.age = config.get_default_age(&self.gender)
        }
    }
}
