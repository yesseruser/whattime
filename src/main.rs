mod tests;
mod time;

use crate::time::Time;
use chrono::Local;

fn main() {
    let datetime = Local::now().naive_local();
    let time = Time::from_datetime(&datetime);
    let formatted = time.to_string();
    println!("It's {formatted}");
}
