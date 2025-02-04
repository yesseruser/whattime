use chrono::{DateTime, Local, Timelike};

pub struct Time{
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}

impl Time{
    pub fn new(hours: u32, minutes: u32, seconds: u32) -> Time {
        Time{hours, minutes, seconds}
    }

    pub fn from_datetime(datetime: &DateTime<Local>) -> Time {
        Time::new(datetime.hour(), datetime.minute(), datetime.second())
    }

    pub fn to_string(&self) -> String {
        format!("{:0>2}:{:0>2}:{:0>2}", self.hours, self.minutes, self.seconds)
    }
}