fn main() {
    println!("Temperature converter");
    println!("To Celsius: {}", fahrenheit_to_celsius(13).to_string());
    println!("To Fahrenheit: {}", celsius_to_fahrenheit(13).to_string());
}

fn fahrenheit_to_celsius(fahrenheit: i8) -> i8 {
  return (fahrenheit - 32) * 5 / 9;
}

fn celsius_to_fahrenheit(celsius: i8) -> i8 {
  return (celsius * 9 / 5) + 32;
}