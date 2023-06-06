#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);
fn main() {
    let mut black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let Color(mut r, g, b) = black;

    r = 254;
    black.0 = 123;

    println!("Black color: {:#?}", black);
    println!("RGB: {}, {}, {}", r, g, b);
    println!("Origin point: {:#?}", origin);
}
