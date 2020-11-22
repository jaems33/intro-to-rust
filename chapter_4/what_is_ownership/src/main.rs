fn takes_ownership(str: String) {
  println!("I have {} and I don't give it back", str);
}

fn takes_and_gives_back(str: String) -> String {
  println!("I have {} but I'll return it", str);
  str
}

fn makes_copy(num: i32){
  println!("I make a copy of {} because it has a Copy trait.", num);
}

fn main() {

  // Example of a string literal, stored on a stack because at compile time we work
  // with a known fixed sized
  let _str_literal = "Some String"; 

  // We can make a deep copy and the original won't be affected by it
  let _copy_literal = _str_literal;
  println!("Str literal {}", _str_literal);

  // Bind 10 to x, make a copy of x, and bind it to y
  // x can still be referred to because it has the Copy trait
  let x = 10;
  let _y = x;

  println!("Value of x pre function call: {}", x);
  makes_copy(x);
  println!("Value of x post function call: {}", x);

  // String::from requests memory on the heap
  // Once it goes out of scope, memory is returned
  let mut s = String::from("Some String");
  s.push_str(" is mutable.");
  println!("s is {}", s);

  /*
   String is represented by ptr, len, capacity and it's character values are stored on the heap.
   
   s1 is allocated to the heap
   s2 gains ownership of the values on the heap s1 points to
   
   If s1 and s2 were allowed to have ownership, they'd both release ownership
   at the end of the call block which would result in a double free error

   Instead, Rust will immediately consider s1 to not be valid. 
   Ownership is moved, whereby s1 is moved into s2.
  */ 
  let s1 = String::from("Example String");
  let s2 = s1;
  println!("s2 is {}", s2);
  
  // To allow s2 to still be usable, we do a deep copy of its values for another
  // variable to use. Keep in mind cloning may be expensive.
  let _s3 = s2.clone();
  println!("s2 is {}", s2);

  let s4 = String::from("String From Main");
  takes_ownership(s4);

  let mut s5 = String::from("String From Main");
  s5 = takes_and_gives_back(s5);
  println!("s5 is {}", s5);
} 
