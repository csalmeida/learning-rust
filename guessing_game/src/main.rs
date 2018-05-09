// Imports a crate into the file, an external dependency.
extern crate rand;

// To bring other parts of Rust into scope a user statement needs to be written.
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // Prints a line, in Rust this is a macro.
    println!("Guess the number!");
    println!("Please input your guess.");

    // Variable that is not mutable (cannot be changed after assigned).
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);
    
    loop {
        // Creates a mutable empty string variable that acts as a place holder for a guess.
        let mut guess = String::new();
        println!("Please input your guess.");
        // Uses io module to allow input to take place.
        // Makes use of placeholder variable.
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Converts guess from string to unsigned integer.
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");
            
        // Prints guess.
        println!("You guessed: {}", guess);

        // Match runs the line that fits its pattern. This match has three arms each with it's own pattern, less, greater, equal.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
