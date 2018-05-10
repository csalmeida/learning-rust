fn a_function() {
    println!("An example function.");
}

// Arguments must have a declared type.
fn sum(x: i32, y: i32) {
    println!("The result is {}.", x + y);
}

fn main() {
    a_function();
    sum(20,77);
}
