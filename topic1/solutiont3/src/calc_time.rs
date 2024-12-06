mod json_io;
use std::collections::HashMap;

use json_io::json_parse_from_current_dir;
use serde::Deserialize;

#[derive(Debug)]
struct ISOTime {
    year: i32,
    month: i32,
    day: i32,
    weekday: i32, // 周日为6
    week_of_year: i32,
    days_left_of_year: i32,
}
impl ISOTime {
    fn result(&self, days_from_next_lunar: i32) -> String {
        format!(
            "{},{},{}",
            self.week_of_year, self.days_left_of_year, days_from_next_lunar
        )
    }
}
#[derive(Deserialize, Debug)]
struct SolarDate {
    year: i32,
    month: i32,
    day: i32,
}
impl SolarDate {
    fn from_str(time: &str) -> Self {
        SolarDate {
            year: time[0..4].to_string().parse().unwrap(),
            month: time[5..7].to_string().parse().unwrap(),
            day: time[8..10].to_string().parse().unwrap(),
        }
    }
    fn year(&self) -> i32 {
        self.year
    }
    fn from_ymd(year: i32, month: i32, day: i32) -> Self {
        SolarDate { year, month, day }
    }
    fn to_julian(&self) -> f64 {
        let a = (14 - self.month) / 12;
        let y = self.year + 4800 - a;
        let m = self.month + 12 * a - 3;
        (self.day + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045) as f64
    }
    fn from_julian(julian_day: f64) -> Self {
        let mut jd = julian_day + 0.5;
        let z = jd.floor();
        let f = jd - z;

        let a = if z >= 2299161.0 {
            let alpha = ((z - 1867216.25) / 36524.25).floor();
            z + 1.0 + alpha - (alpha / 4.0).floor()
        } else {
            z
        };

        let b = a + 1524.0;
        let c = ((b - 122.1) / 365.25).floor();
        let d = (365.25 * c).floor();
        let e = ((b - d) / 30.6001).floor();

        let day = b - d - (30.6001 * e).floor() + f;
        let month = if e < 14.0 { e - 1.0 } else { e - 13.0 };
        let year = if month > 2.0 { c - 4716.0 } else { c - 4715.0 };
        SolarDate {
            year: year as i32,
            month: month as i32,
            day: day as i32,
        }
    }
    // 周日为6
    fn weekday(&self) -> i32 {
        ((self.to_julian() + 0.5) % 7.0) as i32
    }
    fn year_first_day(&self) -> SolarDate {
        SolarDate::from_ymd(self.year, 1, 1)
    }
    fn year_last_day(&self) -> SolarDate {
        SolarDate::from_ymd(self.year, 12, 31)
    }
    fn date_delta(a: &SolarDate, b: &SolarDate) -> i32 {
        (a.to_julian().floor() - b.to_julian().floor()) as i32
    }
    fn week_of_year(&self) -> i32 {
        let first_day = self.year_first_day();
        let last_day = self.year_last_day();
        if last_day.weekday() < 3 && SolarDate::date_delta(&last_day, &self) <= last_day.weekday() {
            return 1;
        } else {
            let mut week_count =
                (SolarDate::date_delta(&self, &first_day) + first_day.weekday()) / 7 + 1;
            if first_day.weekday() > 3 {
                week_count -= 1;
                if week_count == 0 {
                    return SolarDate::from_ymd(self.year - 1, 12, 31).week_of_year();
                }
            };
            week_count
        }
    }
}
impl Into<ISOTime> for SolarDate {
    fn into(self) -> ISOTime {
        let weekday = self.weekday();
        let week_of_year = self.week_of_year();
        let days_left_of_year = SolarDate::date_delta(&SolarDate::year_last_day(&self), &self);
        ISOTime {
            year: self.year,
            month: self.month,
            day: self.day,
            weekday,
            week_of_year,
            days_left_of_year,
        }
    }
}
impl From<&ISOTime> for SolarDate {
    fn from(iso: &ISOTime) -> Self {
        SolarDate {
            year: iso.year,
            month: iso.month,
            day: iso.day,
        }
    }
}
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
    // 基于儒略历(即用从某日开始的数字制作日期映射)
    let new_year: HashMap<i32, SolarDate> =
        json_parse_from_current_dir("lunar_new_year_dates.json");
    let t = SolarDate::from_str(time);
    let next_lunar = find_next_lunar_festvial(&new_year, &t);
    let days_from_next_lunar = SolarDate::date_delta(&next_lunar, &t) - 1;
    let iso: ISOTime = t.into();
    iso.result(days_from_next_lunar)
}
