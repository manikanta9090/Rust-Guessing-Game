use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Generate random secret number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // Create a mutable string to store user input
        let mut guess = String::new();

        // Read user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert string input to number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Ignore invalid input and continue loop
        };

        println!("You guessed: {guess}");

        // Compare guess with secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // Exit loop when correct
            }
        }
    }
}
