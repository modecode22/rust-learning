use std::thread::sleep;
use std::time::Duration;

const WORK_TIME: u64 = 25; // 25 minutes for work
const BREAK_TIME: u64 = 5; // 5 minutes for break

fn main() {
    loop {
        start_timer("Work", WORK_TIME);
        start_timer("Break", BREAK_TIME);
    }
}

fn start_timer(label: &str, minutes: u64) {
    println!("Start {} for {} minutes", label, minutes);

    for i in (1..=minutes).rev() {
        println!("{}: {} minutes left", label, i);
        sleep(Duration::from_secs(60));
    }

    println!("{} time is over!", label);
}
