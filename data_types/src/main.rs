fn main() {
    // Number types
    println!("Printing number types:");
    let x = 1;
    let y: u8 = 2;
    let z = 3.1;
    let n: f32 = 4.2;
    println!("x:{}\ny:{}\nz:{}\nn:{}\n", x,y,z,n);

    // Boolean type
    println!("Printing Boolean types:");
    let foo = true;
    let bar: bool = false;
    println!("Foo is {} and bar is {}.\n", foo, bar);

    // Character type
    // This requires to be set in single quotes.
    println!("Printing character types:");
    let emoji_bow = '😄';
    let emoji_horse: char = '🏇';
    println!("Here's two character types: {}{}\n", emoji_bow,emoji_horse);

    // Compound types
    // Grouping multiple types together using tuples
    println!("Printing compound types:");
    let tup: (u8, i32, char, f32, bool) = (255, 32, '🙌', 20.77, true);
    println!("Here's part of a tuple value: {}\n", tup.2);

    // Arrays can hold multiple values of the SAME type and cannot grow or shrink after being declared.
    let arr = [02,11,2077];
    println!("Retrieving values from an array: {}.{}.{}\n", arr[0], arr[1], arr[2]);
}
