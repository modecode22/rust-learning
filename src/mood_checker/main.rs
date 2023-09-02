
use std::io;

fn main() {
    println!("How do you feel today? (good/bad)");
    
    let mut mood = String::new();
    io::stdin().read_line(&mut mood).expect("Failed to read line");
    let mood = mood.trim();

    if mood == "bad" {
        println!("Remember: Every cloud has a silver lining!");
    } else {
        println!("Great to hear that!");
    }
}
