// To bring a library or other parts of Rust into scope a user statement needs to be written.
use std::io;

fn main() {
    // Prints a line, in Rust this is a macro.
    println!("Guess the number!");
    println!("Please input your guess.");

    // Creates a mutable empty string variable that acts as a place holder for a guess.
    // Adds another variable that is not mutable (cannot be changed after assigned).
    let mut guess = String::new();
    let name = "Cristiano";

    // Uses io module to allow input to take place.
    // Makes use of placeholder variable.
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    // Prints guess.
    println!("{} you guessed: {}", name, guess);
}
