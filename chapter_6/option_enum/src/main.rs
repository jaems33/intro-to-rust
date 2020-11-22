/*
  Option Enum encodes the idea of a value being present or absent.
  enum is Option<T>

  This is similar to the library implementation
  enum QuasiOption<T> {
    Some(T),
    None,
  }
*/

fn divide(num: i32, den: i32) -> Option<i32> {
  if den == 0 {
    return Option::None
  }
  Option::Some(num / den)
}

fn main() {

  /* 
    The type of None must be provided as Rust can't infer the type
  */
  let _number: Option<i32> = None;
  
  let test_num = divide(10, 0);
  match test_num {
    Some(x) => println!("Total is: {}", x),
    None => println!("Can't divide by zero.")
  }

  let test_num = divide(10, 8);
  match test_num {
    Some(x) => println!("Total is: {}", x),
    None => println!("Can't divide by zero.")
  }

}
