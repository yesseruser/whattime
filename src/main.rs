mod time;

use chrono::Local;
use crate::time::Time;

fn main() {
    let datetime = Local::now().naive_local();
    let time = Time::from_datetime(&datetime);
    let formatted = time.to_string();
    println!("It's {formatted}");
}