fn main() {
  // Declaring and using a variable returns the assigned value as expected.
  let x = 5;
  println!("The inital value of x is: {x}");

  // Redeclaring the variable shadows the first assignment and points it to another value in the current scope.
  let x = x + 1;

  {
      // Shadows variable again but only in the scope of this block.
      let x = x * 2;
      println!("The value of x in the inner scope is: {x}");
  }

  // Accessing the variable here will still return 6 despite being shadowed in the inner scope.
  println!("The value of x is: {x}");


  // Same case here, declaring and using a variable returns the assigned value as expected.
  let y = 0;
  println!("The inital value of y is: {y}");

  // The shadowed variable does not need to make use of the previous value, a completely new value will override it in the current scope.
  let y = 1;
  println!("The inital value of y is: {y}");
}