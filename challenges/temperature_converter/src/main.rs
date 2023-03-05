// This program converts temperature from Celsius to Fahrenheit and vice-versa.
// A few nice to have features would be:
//  - Being able to run multiple values without exiting the program.
//  - Being able to switch between converting modes without exiting the program.
//  - Being able to validate if the user has entered a number and request another value if there's an error, intead of panicking.

use std::io;

fn main() {
    println!("Temperature Converter");

    // Pick which temperature to convert to:
    println!("Please type to which unit you would like the temperature to be converted to.");
    println!("Try 'celsius' or 'c' and 'fahrenheit' or 'f'");

    let mut temperature = String::new();

    // Request input until a supported value has been entered.
    let mut has_supported_value: bool = false;
    while has_supported_value == false {
        let mut convert_to_temperature = String::new();

        // Requests user input:
        io::stdin()
            .read_line(&mut convert_to_temperature)
            .expect("Failed to read line");

        // Removes any trailing new lines in input:
        convert_to_temperature = convert_to_temperature.trim().to_string();

        if convert_to_temperature.to_string() == "celsius"
            || convert_to_temperature.to_string() == "c"
        {
            println!("Please enter your value in Fahrenheit:");
            // Reads temperature entered by user.
            io::stdin()
                .read_line(&mut temperature)
                .expect("Failed to read line");

            // Need to convert the input back into a number:
            let temperature = temperature.trim().parse().unwrap();
            println!(
                "{} Fahrenheit is {} Celsius.",
                temperature,
                fahrenheit_to_celsius(temperature)
            );
            has_supported_value = true;
        } else if convert_to_temperature == "fahrenheit" || convert_to_temperature == "f" {
            println!("Please enter your value in Celsius:");
            // Reads temperature entered by user.
            io::stdin()
                .read_line(&mut temperature)
                .expect("Failed to read line");

            // Need to convert the input back into a number:
            let temperature = temperature.trim().parse().unwrap();
            println!(
                "{} Celsius is {} Fahrenheit",
                temperature,
                celsius_to_fahrenheit(temperature)
            );
            has_supported_value = true;
        } else {
            println!("Temperature type not recognized.");
            println!("Try 'celsius' or 'c' and 'fahrenheit' or 'f'");
        }
    }
    println!("Ended execution.");
}

fn fahrenheit_to_celsius(fahrenheit: i32) -> i32 {
    return (fahrenheit - 32) * 5 / 9;
}

fn celsius_to_fahrenheit(celsius: i32) -> i32 {
    return (celsius * 9 / 5) + 32;
}
