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
}