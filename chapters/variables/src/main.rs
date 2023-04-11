fn main() {
    let mut x = 5;
    println!("\nThe value of mutable x is: {}\n", x);

    x = 6;
    println!("The value of mutable x was changed to: {}\n", x);

    let y = 10;
    println!("The value of immutable y is: {}\n", y);

    let y = y + 1;
    let y = y * 3;
    println!(
        "Variable y can be redeclared to change its value and keep immutability: {}\n",
        y
    );

    const MAX_POINTS: u32 = 100_000;
    println!(
        "A constant is always immutable.\nThis one has a value of {}\n",
        MAX_POINTS
    );
}
