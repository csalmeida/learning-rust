// This program generates the nth Fibonacci number.
// Notes:
// - The nth number is 1-based so the 54th number in the sequence is 53316291173 but
//   in an 0-based sequence it would be F53 and most online tables count it that way.
//   For example the output of 54 would be: "The 54th number (F53) in the Fibonacci sequence is 53316291173."
// Improvements:
// - Calculate the number using a formula with the Golden Ratio which in theory it would not require iterations:
// - let golden_ratio: f64 = 1.618033988749894;

use std::io;

fn main() {
    println!("Generate a Fibonacci number.");
    println!("Pick the nth number of the sequence to find it. For example, try '5' or '34'.");

    let mut nth_number = String::new();
    // Requests user input:
    io::stdin()
        .read_line(&mut nth_number)
        .expect("Failed to read line");

    // Parse string input into a number.
    let nth_number: i64 = nth_number.trim().parse().unwrap();

    let max_number = 186;
    if nth_number >= max_number {
        println!(
            "The {} number in the sequence is out of bounds.",
            get_number_suffix(nth_number)
        );
        println!("The program cannot generate a result that high.");
        return;
    }

    // Print the result:
    println!(
        "The {} number (F{}) in the Fibonacci sequence is {}.",
        get_number_suffix(nth_number),
        nth_number - 1,
        calculate_fibonnaci(nth_number)
    );
}

/*
 * This function calculates the nth Fibonacci number.
 * It skips the first few numbers in the sequence (0, 1 and 1) and
 * calculates the rest from that point onwards.
 */
fn calculate_fibonnaci(nth_number: i64) -> i128 {
    let mut index: i64 = 0;
    let mut current_number: i128 = 0;

    // The first number of the sequence is always 0.
    if nth_number == 1 {
        return 0;
    }

    // The second number of the sequence is always 1.
    if nth_number == 2 {
        return 1;
    }

    // The third number of the sequence is always 1.
    if nth_number == 3 {
        return 1;
    }

    // Quando escreves no terminal 1, queres o 1st number of Fib
    // Expected: 0, Result: 1.

    // The sum of these numbers will give the current Fibonnachi
    let mut previous_number: i128 = 0;
    let mut second_to_last_number: i128 = 1;

    // Offsets a few iterations to catchup to missed inital numbers (0,1,1).
    let offset_nth_number = nth_number - 2;
    while index != offset_nth_number {
        current_number = previous_number + second_to_last_number;

        previous_number = second_to_last_number;
        second_to_last_number = current_number;

        index = index + 1;
    }

    return current_number;
}

/*
 * Adds a suffix to the number provided.
 * For example, 1 will become 1st, 2, 2nd, 3, 3rd and so on.
 */
fn get_number_suffix(number: i64) -> String {
    let mut number_with_suffix = number.to_string();
    let last_character = number_with_suffix.chars().last().unwrap();
    let is_in_range: bool = number < 10 || number > 20;

    if last_character == "1".chars().last().unwrap() && is_in_range {
        number_with_suffix = number_with_suffix + "st";
    } else if last_character == "2".chars().last().unwrap() && is_in_range {
        number_with_suffix = number_with_suffix + "nd";
    } else if last_character == "3".chars().last().unwrap() && is_in_range {
        number_with_suffix = number_with_suffix + "rd";
    } else {
        number_with_suffix = number_with_suffix + "th";
    }

    return number_with_suffix;
}
