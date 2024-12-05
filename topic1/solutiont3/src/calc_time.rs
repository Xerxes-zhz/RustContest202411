#[derive(Debug)]
struct ISOTime {
    year: u32,
    month: u32,
    day: u32,
}
#[derive(Debug)]
struct NormalTime {
    year: u32,
    month: u32,
    day: u32,
}
impl NormalTime {
    fn from_str(time: &str) -> Self {
        NormalTime {
            year: time[0..4].to_string().parse().unwrap(),
            month: time[5..7].to_string().parse().unwrap(),
            day: time[8..10].to_string().parse().unwrap(),
        }
    }
}
impl Into<ISOTime> for NormalTime {
    fn into(self) -> ISOTime {
        ISOTime {
            year: self.year,
            month: self.month,
            day: self.day,
        }
    }
}

pub fn time_info(time: &str) -> String {
    // 基于儒略历(即用从某日开始的数字制作日期映射)
    //
    let t = NormalTime::from_str(time);
    let iso: ISOTime = t.into();

    println!("{:?}", iso);
    "".to_string()
}
