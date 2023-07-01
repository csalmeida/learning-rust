#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);
fn main() {
    // Tuple structures can be defined do capture a number of values.
    let mut black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Tuples can be destructured to access individual values of the structure.
    let Color(mut red, green, blue) = black;

    // If mutable these can be changed.
    // The r variable is the same as black.0 but one can be seen as more readable.
    red = 254;
    black.0 = 123;

    println!("Black color: {:#?}", black);
    println!("RGB: {}, {}, {}", red, green, blue);
    println!("Origin point: {:#?}", origin);
}
