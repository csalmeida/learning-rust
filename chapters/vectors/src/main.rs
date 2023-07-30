fn main() {
    // Creating a Vector with inital values:
    let numbers = vec![1, 2, 3];
    println!("Numbers vector is {:#?}", numbers);

    // Creating a Vector and updating it later:
    let mut years = Vec::new();
    years.push(1984);
    years.push(2020);
    years.push(2023);
    years.push(2077);
    years.push(2099);

    println!("Years vector is {:#?}", years);

    // Accessing a vector this way does not allow the program to panic.
    let second_year: Option<&i32> = years.get(1);
    match second_year {
        Some(value) => println!("The first year in the vector is {value}"),
        None => println!("There is no first element for this vector."),
    };

    // In some cases we would like the program to panic if the index does not exist in the list:
    // let tenth_year = &years[10];

    // Vectors only allow values of the same type to be included.
    // Use an Enum to allow multiple types of values.
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("Spreadsheet row vector is {:#?}", row);
}
