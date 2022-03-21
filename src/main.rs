use std::io;
// Import a crate from Crate.io that generates random numbers.
use rand::Rng;

fn main() {
    println!("Guess the number!");

    /*
     * Call the crate, read docs to know how. Use cargo doc --open.
    */ 
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Secret number is {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
