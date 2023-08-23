use std::io;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Enter the number of seconds for the reminder:");

    let mut seconds = String::new();
    io::stdin().read_line(&mut seconds).expect("Failed to read line");
    let seconds: u64 = seconds.trim().parse().expect("Invalid number");

    println!("Reminder set for {} seconds...", seconds);
    sleep(Duration::from_secs(seconds));
    println!("Time's up! Don't forget your task!");
}
