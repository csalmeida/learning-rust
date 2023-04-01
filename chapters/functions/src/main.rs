fn a_function() {
    println!("An example function.");
}

// Arguments must have a declared type.
fn sum(x: i32, y: i32) {
    println!("The result is {}.", x + y);
}

// Explicitly returns a signed 32 bit integer.
fn five() -> i32 {
    5
}

// Takes a signed 32 bit integer has an argument, returns i32.
fn plus_one(x: i32) -> i32 {
    x + 1
}

// Define functions anywhere, run on main.
fn main() {
    a_function();
    sum(20,77);
    println!("This function returns a value: {}", five());
    println!("This function returns a value and allows an argument: {}", plus_one(16));
}
