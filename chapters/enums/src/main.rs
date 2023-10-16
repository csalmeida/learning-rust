use std::error::Error;

enum Messages {
    Hello,
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    ChangePosition(Point),
}

impl Messages {
    fn print_data(&self) {
        match self {
            Messages::Hello => println!("Hello"),
            Messages::Quit => println!("Quit"),
            Messages::ChangeColor(r, g, b) => println!("ChangeColor({},{},{})", r, b, g),
            Messages::Move { x, y } => println!("Move to ({}, {})", x, y),
            Messages::ChangePosition(Point { x, y }) => {
                println!("Change position to ({}, {})", x, y)
            }
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Testing all message variants:
    Messages::Hello.print_data();
    Messages::Quit.print_data();
    Messages::ChangeColor(143, 54, 25).print_data();
    Messages::Move { x: -33, y: 56 }.print_data();
    Messages::ChangePosition(Point { x: 0, y: 15 }).print_data();

    // Return an option type and deal with it.
    let my_number_option: Option<u8> = Some(6);

    // Because Option is an Enum, we have to address the case for each variant.
    match my_number_option {
        Some(value) => println!("let my_number_option: Option<u8> = {:?}", value),
        _ => (),
    }

    // Using this function to store the number nine with the value that results from an option.
    let mut nine = 0;

    // However, in this case we want to access the number inside Some but do nothing if there's no value.
    // In these cases we can use if let expressions to run if the number is actually present and we can then access the value inside the block:
    if let Some(value) = my_number_option {
        println!("if let Some(value) = my_number_option is {:?}", value);
        nine = value + 3;
    }

    println!("let mut nine = {:?}", nine);

    return Ok(());
}
