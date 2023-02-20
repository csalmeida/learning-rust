fn main() {
  let mut x = 3;
  let y = 7;

  if x == 3 {
    x = x + y;
  } else if x < 3 {
    x = 15;
  } else {
    x = 0;
  }
  
  println!("x is equal to: {}", x);

  // Additional to the long form, an if statement can be added to a let statement:
  let condition = true;
  let number = if condition { 5 } else { 6 };

  println!("The value of number is: {number}");

  // Loops can return a value using the break keyword.
  let mut counter = 0;

  let result = loop {
      counter += 1;

      if counter == 10 {
          break counter * 2;
      }
  };

  println!("The result is {result}");


  // Loops can be labeled starting with a single quote. The loop can be broken within another loop using the label.
  let mut count = 0;
  'counting_up: loop {
      println!("count = {count}");
      let mut remaining = 10;

      loop {
          println!("remaining = {remaining}");
          if remaining == 9 {
              break;
          }
          if count == 2 {
              break 'counting_up;
          }
          remaining -= 1;
      }

      count += 1;
  }
  println!("End count = {count}");

  // Use while loops for conditional iteration:
  let mut number = 3;

  while number != 0 {
      println!("{number}!");

      number -= 1;
  }

  println!("LIFTOFF!!!");

  // To loop the contents of an array it tends to be less error prone to use a for loop:
  let a = [10, 20, 30, 40, 50];

  for element in a {
      println!("the value is: {element}");
  }

  // To run code limited number of times it's recommended to use a for loop with a range:
  for number in (1..4).rev() {
    println!("{number}!");
  }
  println!("LIFTOFF!!!");
}