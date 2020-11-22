/*
  IpAddress design using structs and enums
*/

#[derive(Debug)]
enum IpAddressKind {
  V4, V6
}

struct IpAddress {
  kind: IpAddressKind,
  address: String,
}

/*
  IpAddress design just using enums
*/
enum IpAddressV2 {
  V4(u8, u8, u8, u8),
  V6(String)
}

/*
  Similar to a struct except variants are grouped together
  in the Message type
*/
#[derive(Debug)]
enum Message {
  Quit,
  Move{x: i32, y: i32},
  Write(String),
  ChangeColor(i32, i32, i32)
}

impl Message {
  fn call(&self) {
    println!("Calling: {:?}", self);
  }
}

fn _route(ip: IpAddressKind){
  println!("Does something: {:?}", ip);
}

fn main() {
  
  let _home1 = IpAddress {
    kind: IpAddressKind::V4,
    address: String::from("128.128.0.1"),
  };
  
  let _home1 = IpAddress {
    kind: IpAddressKind::V6,
    address: String::from("::1")
  };

  println!("IpAddress Info: {} {:?}", _home1.address, _home1.kind);

  let _home2 = IpAddressV2::V4(128, 128, 0, 1);
  let _home2 = IpAddressV2::V6(String::from("::1"));

  let _msg = Message::Write(String::from("Hello World"));
  _msg.call();

  let _msg = Message::Move {
    x: 12, y: 21
  };
  
  let _msg = Message::ChangeColor(23, 23, 23);

  let _msg = Message::Quit;
  _msg.call();
  
}
