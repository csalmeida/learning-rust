fn main() {
    // Writting an if statement.
    // If it gets long consider refactoring into a match.
    let x: u16 = 254;
    if x > 255 {
        println!("x is greater than 255");
    } else if x < 255 && x > 200 {
        println!("x is within the 200-255 threshold");
    } else {
        println!("x is less than 255");
    }

    let condition: bool = false;
    let number: u8 = if condition {
        1
    } else {
        0
    };
    println!("The number set with an if expression is: {}", number);
}
