mod tests;
mod time;

use crate::time::Time;
use chrono::Local;

fn main() {
    let naive_time = Local::now().naive_local().time();
    let time = Time::from_naive_time(&naive_time);
    let formatted = time.to_string();
    println!("It's {formatted}!");
}
