use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // Define a secret number between 1 and 100
    // println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // Your guess variable in Sring form
        // mut means it is mutable, leave it out for an immutable variable
        io::stdin()
            .read_line(&mut guess) // Read your guess from stdin 
            .expect("Failed to read line"); // If aan error occured

        let guess: u32 = match guess.trim().parse() { // Turn your String guess into an unsigned 32-bit integer
            Ok(num) => num, // accept a number
            Err(_) => continue, // continue if not a number
        };

        println!("Your guess: {guess}");

        match guess.cmp(&secret_number) { // Compare your guess with the secret number
            Ordering::Less => println!("Guess is too small!"), // Try again if too small
            Ordering::Greater => println!("Guess is too big!"), // try again with too big
            Ordering::Equal => { // Congratulate if correct guess and break the loop
                println!("Correct, good guess");
                break;
            }
        }
    }
}
