use serde::Deserialize;
use std::env::current_dir;
use std::{collections::HashMap, fs};
#[derive(Deserialize, Debug)]
struct IDCard {
    #[serde(rename = "Province")]
    province: String,
    #[serde(rename = "City")]
    city: String,
    #[serde(rename = "County")]
    county: String,
}
impl IDCard {
    pub fn location(&self) -> String {
        format!("{}-{}-{}", self.province, self.city, self.county)
    }
}
fn check_id_mod_11_2(id_card_no: &str) -> bool {
    let mut w = 1;
    let mut sum = 0;
    for (i, a) in id_card_no.chars().rev().enumerate() {
        if let Some(a) = a.to_digit(10) {
            sum += a * w;
        } else {
            if ((a == 'x') || (a == 'X')) && (i == 0) {
                sum += 10;
            } else {
                return false;
            }
        }
        w = w << 1;
        w %= 11;
    }
    sum % 11 == 1
}
pub fn check_id_card(id_card_no: &str) -> String {
    // 分情况讨论
    let data_json_path = current_dir().unwrap().join("data.json");
    let file = fs::read_to_string(data_json_path).unwrap();
    let id_card_map: HashMap<String, IDCard> = serde_json::from_str(&file).unwrap();
    let mut gender = "女";
    let mut y = "".to_string();
    let mut m = "";
    let mut d = "";
    let id_card: Option<&IDCard> = {
        if id_card_no.len() == 18 {
            // 校验码
            if check_id_mod_11_2(id_card_no) {
                if &id_card_no.chars().nth(16).unwrap().to_digit(10).unwrap() % 2 == 1 {
                    gender = "男";
                }
                y = id_card_no[6..10].to_string();
                m = &id_card_no[10..12];
                d = &id_card_no[12..14];
                id_card_map.get(&id_card_no[0..6])
            } else {
                None
            }
        } else if id_card_no.len() == 15 {
            if &id_card_no.chars().nth(14).unwrap().to_digit(10).unwrap() % 2 == 1 {
                gender = "男";
            }
            // 15位19开头
            y = "19".to_string();
            y.push_str(&id_card_no[6..8]);
            m = &id_card_no[8..10];
            d = &id_card_no[10..12];
            id_card_map.get(&id_card_no[0..6])
        } else {
            None
        }
    };
    if let Some(ic) = id_card {
        format!(
            "身份证号码正确,{},{}年{}月{}日,{}",
            gender,
            y,
            m,
            d,
            ic.location()
        )
    } else {
        "身份证号码错误".to_string()
    }
}
