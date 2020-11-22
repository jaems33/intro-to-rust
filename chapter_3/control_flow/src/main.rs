fn main() {
  const NUM:i32 = 10;
  let number = 3;
  
  // Blocks of code in conditions are sometimes called 'arms'
  if number == NUM {
    println!("Equals {}", NUM);
  } else {
    println!("Does not equal {}", NUM);
  }

  // Numbers are not converted to falsy
  if number != 0 {
    println!("Number is not zero.");
  } else {
    println!("Number is zero.");
  }

  // Multiple conditions
  if number == 1 {
    println!("Number equals 1.");
  } else if number == 2 {
    println!("Number equals 2.");
  } else {
    println!("Number is 3 or greater.");
  }

  // Conditions can be evaluated on one line, but the outcomes must be of the same type
  let x = if number > 10 { 20 } else { 0 };
  println!("x is {}", x);
  
  // Conditions with loops, notice how we can return the result
  // by breaking and including an expression
  let mut counter = 0;
  let result = loop {
    counter += 1;
    if counter == 10 {
      break counter + 13;
    }
  };

  println!("Counter equals: {}", result);

  counter = 0;
  while counter < 3 {
    counter += 1;
  };

  // For loop, two ways

  let values = [10, 20, 30, 40, 50];

  for value in values.iter(){
    println!("Value: {}", value);
  };

  let mut a1:u128 = 1;
  let mut a2:u128 = 1; 
  
  let upperlimit = 128;
  for _ in 0..upperlimit {
    let temp = a2;
    a2 = a1 + a2;
    a1 = temp;
  }
  println!("fib: {}", a2);

}