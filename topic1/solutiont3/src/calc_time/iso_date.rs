#[derive(Debug)]
pub struct ISOTime {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub weekday: i32, // 周日为6
    pub week_of_year: i32,
    pub days_left_of_year: i32,
}
impl ISOTime {
    pub fn result(&self, days_from_next_lunar: i32) -> String {
        format!(
            "{},{},{}",
            self.week_of_year, self.days_left_of_year, days_from_next_lunar
        )
    }
}
