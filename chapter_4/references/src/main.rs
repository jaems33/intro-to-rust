/*
  Ampersand appears before datatype, indicates a reference
  The value of str will not disappear even when str goes out of scope
  This is an example of 'borrowing', once the function is done it has to give ownership back
  This is an example of a non-mutable reference
*/
fn length(str: &String) -> usize {
  str.len()
}

fn add_period(str: &mut String) {
  str.push_str(".");
}

/* 
  This creates a dangling reference because the String is created in the function
  but the reference is returned outside of the function. Once the function is complete,
  the String will be deallocated. Rust will throw an error if this function is uncommented.

fn dangling_function() -> &String {
  let str = String::from("Sample String");
  &str
}
*/

/*
  This function moves ownership of the String outside of the function
*/
fn non_dangling_function() -> String {
  let str = String::from("Sample String");
  str
}

fn main() {
  /* 
    References allow one to temporarily give ownership
    of an object to another entity 
  */
  let s1 = String::from("This is on the heap");

  /*
    Ampersand before variable name indicates a reference
    The function cannot mutate s1.
  */
  let length = length(&s1);
  println!("The string is: {}", s1);
  println!("Length is: {}", length);

  /* 
    Variables can be passed to function as mutable references
    provided the variable is mutable and passed as a mutable reference.
  */
  let mut s2 = String::from("This is a sentence without a period");
  add_period(&mut s2);
  println!("{}", s2);

  /* 
    Cannot have more than one mutable reference to an object at the same time.
    Benefit is to prevent data races at compile time. New references made
    in curly braces (new scopes) are fine.
  */
  let _s2_borrow = &mut s2;
  let _s3_borrow = &mut s2;

  //_s2_borrow.push_str("Hi"); would result in an error

  /* 
    You can have multiple non-mutable references at once, but not both
    non-mutable and mutable at once.
  */
  let _s4_borrow = &s2;
  let _s5_borrow = &s2;
  println!("{}, {}", _s4_borrow, _s5_borrow);
  let _s6_borrow = &mut s2;
  //println!("{}, {}", _s4_borrow, _s5_borrow); would result in an error

  let _new = non_dangling_function();
}
