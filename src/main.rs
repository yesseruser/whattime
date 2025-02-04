use chrono::Local;

fn main() {
    let datetime = Local::now();
    println!("It's {}", datetime.format("%H:%M:%S"));
}
