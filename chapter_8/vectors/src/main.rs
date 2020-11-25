fn main() {
  let mut v1: Vec<i32> = Vec::new();
  
  let v2 = vec![1, 2, 3];

  v1.push(7);
  println!("{}", v1[0]);
  
  let first = &v1[0];

  //v1.push(9); Can't do this because first is a borrow
  // and if you push onto a vector it may re-allocate and change the position
  // of first
  println!("First is: {}", first);
  v1.push(9);

  match v1.get(0){
    Some(value) => println!("Value is: {}", value),
    None => println!("No first element."),
  }

  let does_not_exist = v2.get(1000);
  match does_not_exist {
    Some(value) => println!("Value is {}", value),
    None => println!("No such index."),
  }

  let mut v = vec![3, 7, 13];
  for i in &mut v {
    *i += 10;
    println!("{}", i);
  }

  /*
    Using enums to store different types in a vector
  */

  enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  let row = vec![
    SpreadSheetCell::Int(3),
    SpreadSheetCell::Float(3.93), 
    SpreadSheetCell::Text(String::from("Some String")),
  ];
  
}

