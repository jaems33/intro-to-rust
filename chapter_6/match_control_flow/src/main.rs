/*
  Match is one of the most powerful tools in Rust
  Allowing coders to flow to code based on the pattern match

  Matches are exhaustive so we have to account for every possible case.
  Not including say one of the types of Enum types would result in an error

  The match is like a coin sorting machine 
*/
#[derive(Debug)]
enum Color {
  Golden,
  Red
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
  Loonie,
  Toonie(Color),
}

fn value_in_cents(coin: &Coin) -> u8 {
  /*
    if needs to return a boolean value, but match can be of any type
    
    An 'arm' has two parts: a pattern and code
    The Arrow operator separates the pattern and the code to run
    Curly braces can be used for multiple lines of code
  */
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
    Coin::Loonie => 100,
    Coin::Toonie(color) => {
      println!("Wow a toonie? You're rich!");
      println!("Plus it has {:?} color!", color);
      200
    }
  }
}

/*
  Concise control flow with if let
  Less exhaustive than match, but good for small checks and less boilerplate
*/
fn check_if_penny(coin: &Coin){
  if let Coin::Penny = coin {
    println!("Found a penny!");
  } else {
    println!("This isn't a penny");
  }
}


fn increment_by_10 (x: Option<i32>) -> Option<i32> {
  /*
    x binds to the value contained in Some
  */
  match x {
    None => None,
    Some(x) => Some(x + 1)
  }
}

fn print_number_up_to_2 (x: i32){
  /*
    Use _ for any value (aka default)
  */
  match x {
    1 => println!("One!"),
    2 => println!("Two!"),
    _ => println!("{}!", x)
  }
}

fn main() {
  value_in_cents(&Coin::Penny);
  value_in_cents(&Coin::Nickel);
  value_in_cents(&Coin::Dime);
  value_in_cents(&Coin::Quarter);
  value_in_cents(&Coin::Loonie);
  value_in_cents(&Coin::Toonie(Color::Golden));
  value_in_cents(&Coin::Toonie(Color::Red));

  let total = increment_by_10(Some(100));
  println!("Total is: {:?}", total);

  print_number_up_to_2(2);
  print_number_up_to_2(3);

  check_if_penny(&Coin::Penny);
  check_if_penny(&Coin::Nickel);
}
