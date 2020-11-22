/*
  Opt into printing out debug information
*/
#[derive(Debug)]
struct Rectangle {
  width: u128,
  height: u128
}

/*
  Implementation block, use &self to reference itself
*/
impl Rectangle {
  fn area (&self) -> u128 {
    return self.height * self.width;
  }
  fn has_greater_area(&self, other: &Rectangle) -> bool {
    if other.area() < self.area() {
      return true;
    }
    false
  }
  /* Associated functions */
  fn square(size: u128) -> Rectangle {
    Rectangle {
      height: size,
      width: size
    }
  }
}

/*
  Multiple seperate implementation blocks can be used for the same struct
  but it's not necessary
*/
impl Rectangle {
  fn perimeter (&self) -> u128 {
    return self.height * 2 + self.width * 2;
  }
}

/*
  Dummy function just to show that you should use a 
  reference when temporarily using custom structs that you
  may use later
*/
fn _double_area(r: &Rectangle) -> u128 {
  r.width * 2
}

fn main() {
  let r1 = Rectangle {
    width: 100,
    height: 10
  };
  
  /*
    Call methods with . not ->
    Rust has automatic referencing/dereferencing. Calling methods uses this.
    When callling obj.some_method() rust adds appropriate syntax,
    e.g. p1.distance(&p2) -> (&p1).distance(&p2);
  */
  println!("Perimeter: {}", r1.perimeter());
  
  /*
    Use debug: {:?}
  */
  println!("Rectangle: {:?}", r1);
  
  /*
    Use debug with formatting: {:#?}
  */
  println!("Rectangle: {:#?}", r1);

  let r2 = Rectangle::square(100);
  println!("r1 Area: {}", r1.area());
  println!("r2 Area: {}", r2.area());
  println!("Does r2 have a larger area than r1? {}", r2.has_greater_area(&r1));

}
