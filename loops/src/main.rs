fn main() {
    // A loop.
    // It will be infinite if no break is present.
    loop {
       println!("Iteration.");
       break;
    }

    let mut number: u8 = 3;
    while number != 0 {
        println!("{}", number);
        number = number - 1;
    }
    println!("LIFTOFF!");

    let decades = [10, 20, 30, 40, 50];
    for decade in decades.iter() {
        println!("Value is: 20{}", decade);
    }
}
