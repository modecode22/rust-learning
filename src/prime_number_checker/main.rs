use std::io;

fn main() {
    println!("Enter a number:");
    
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read line");
    let number: u64 = number.trim().parse().expect("Invalid number");

    if is_prime(number) {
        println!("{} is a prime number.", number);
    } else {
        println!("{} is not a prime number.", number);
    }
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..(n as f64).sqrt() as u64 + 1 {
        if n % i == 0 {
            return false;
        }
    }
    true
}
