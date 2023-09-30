use rand::seq::SliceRandom;

fn main() {
    let cryptos = ["Bitcoin", "Ethereum", "Ripple", "Litecoin", "Cardano"];
    let choice = cryptos.choose(&mut rand::thread_rng()).unwrap();
    println!("You got: {}", choice);
}
