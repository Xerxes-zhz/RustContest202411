use chinese_lunisolar_calendar::{LunarDay, LunarMonth, LunisolarDate, LunisolarYear, SolarDate, SolarYear};
use chrono::Datelike;
use chrono::NaiveDate;
use serde::Serialize;
use serde_json::to_writer_pretty;
use std::collections::HashMap;
use std::fs::File;
use std::env::current_dir;
use std::io::Result;

#[derive(Serialize)]
struct GregorianDate {
    year: u32,
    month: u32,
    day: u32,
}

fn get_lunar_new_year_dates(start_year: i32, end_year: i32) -> HashMap<String, GregorianDate> {
    let mut results = HashMap::new();

    for year in start_year..=end_year {
        let lunar_date = LunisolarDate::from_lunisolar_year_lunar_month_day(
            LunisolarYear::from_solar_year(SolarYear::from_u16(year as u16)).unwrap(),
            LunarMonth::First,
            LunarDay::First,
        )
        .unwrap();  
        let solar_date = SolarDate::from_lunisolar_date(lunar_date);
        let naive_date: NaiveDate = solar_date.into();
        let date = GregorianDate {
            year: naive_date.year() as u32,
            month: naive_date.month() as u32,
            day: naive_date.day() as u32,
        };
        results.insert(year.to_string(), date);
    }

    results
}

fn main() -> Result<()> {
    let start_year = 1901; 
    let end_year = 2100; 

    let dates = get_lunar_new_year_dates(start_year, end_year);

    let path = current_dir().unwrap().join("lunar_new_year_dates.json");
    let file = File::create(path)?;
    to_writer_pretty(file, &dates).expect("Failed to write JSON");

    Ok(())
}


