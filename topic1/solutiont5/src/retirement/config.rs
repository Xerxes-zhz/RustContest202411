use super::retirement_info::{Gender, RetirementInfo};
use serde::Deserialize;
use std::collections::HashMap;
#[derive(Deserialize, Debug)]
pub struct Rule {
    gender: Gender,
    age: i32,
    delay_per_month: f32,
    max_age: i32,
}
#[derive(Deserialize, Debug)]
pub struct Config {
    default: HashMap<Gender, Option<i32>>,
    rules: Vec<Rule>,
    start_year: i32,
    start_month: i32,
}
impl Config {
    pub fn get_default_age(&self, gender: &Gender) -> Option<i32> {
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
    pub fn calc_retire_time(&self, info: &RetirementInfo) -> Option<String> {
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
