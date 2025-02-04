use crate::time::Time;
use chrono::{NaiveTime, Timelike};

macro_rules! assert_time_is_naive_time {
    ($time:expr, $naive_time:expr) => {
        assert_eq!($time.hours, $naive_time.hour());
        assert_eq!($time.minutes, $naive_time.minute());
        assert_eq!($time.seconds, $naive_time.second());
    };
}

#[test]
fn to_string() {
    let time1 = Time::new(0, 0, 0);
    let time2 = Time::new(1, 1, 1);
    let time3 = Time::new(12, 56, 4);
    assert_eq!(time1.to_string(), "00:00:00");
    assert_eq!(time2.to_string(), "01:01:01");
    assert_eq!(time3.to_string(), "12:56:04");
}

#[test]
fn from_naive_time() {
    let naive_time1 = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let naive_time2 = NaiveTime::from_hms_opt(2, 35, 34).unwrap();
    let naive_time3 = NaiveTime::from_hms_opt(15, 12, 3).unwrap();
    let time1 = Time::from_naive_time(&naive_time1);
    let time2 = Time::from_naive_time(&naive_time2);
    let time3 = Time::from_naive_time(&naive_time3);

    assert_time_is_naive_time!(time1, naive_time1);
    assert_time_is_naive_time!(time2, naive_time2);
    assert_time_is_naive_time!(time3, naive_time3);
}
