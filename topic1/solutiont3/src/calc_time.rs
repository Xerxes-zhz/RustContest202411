mod iso_date;
mod json_io;
mod solar_date;

use iso_date::ISOTime;
use json_io::json_parse_from_current_dir;
use solar_date::SolarDate;
use std::collections::HashMap;

fn find_next_lunar_festvial<'a>(
    new_year: &'a HashMap<i32, SolarDate>,
    date: &SolarDate,
) -> &'a SolarDate {
    let lunar_this_year = new_year.get(&date.year()).unwrap();
    if SolarDate::date_delta(date, lunar_this_year) >= 0 {
        new_year.get(&(date.year() + 1)).unwrap()
    } else {
        lunar_this_year
    }
}

pub fn time_info(time: &str) -> String {
    // 基于儒略历制作日期映射
    // 农历采用rust打表
    // 实现ISODate(可直接使用的日期数据结构和result 的fmt)和SolarDate(只有年月日,但包含各种方法)的相互转换(into, from)
    let new_year: HashMap<i32, SolarDate> =
        json_parse_from_current_dir("lunar_new_year_dates.json");
    let t = SolarDate::from_str(time);
    let next_lunar = find_next_lunar_festvial(&new_year, &t);
    let days_from_next_lunar = SolarDate::date_delta(&next_lunar, &t) - 1;
    let iso: ISOTime = t.into();
    iso.result(days_from_next_lunar)
}
