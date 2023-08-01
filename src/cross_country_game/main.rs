use std::io;
use std::cmp::min;
use rand::Rng;

const TOTAL_DISTANCE: usize = 50;

fn main() {
    let mut player1_distance = 0;
    let mut player2_distance = 0;

    loop {
        print_board(player1_distance, player2_distance);

        println!("Press Enter to advance...");
        io::stdin().read_line(&mut String::new()).expect("Failed to read line");

        player1_distance = min(TOTAL_DISTANCE, player1_distance + advance());
        player2_distance = min(TOTAL_DISTANCE, player2_distance + advance());

        if player1_distance == TOTAL_DISTANCE || player2_distance == TOTAL_DISTANCE {
            print_board(player1_distance, player2_distance);
            if player1_distance == TOTAL_DISTANCE {
                println!("Player 1 wins!");
            } else {
                println!("Player 2 wins!");
            }
            break;
        }
    }
}

fn advance() -> usize {
    let random_number = rand::thread_rng().gen_range(1..=10);
    random_number
}

fn print_board(player1_distance: usize, player2_distance: usize) {
    println!("\nCurrent Board:");
    println!("Player 1: {}{}", "-".repeat(player1_distance), "1".to_string());
    println!("Player 2: {}{}", "-".repeat(player2_distance), "2".to_string());
    println!("\n");
}
