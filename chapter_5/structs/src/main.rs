struct User {
  username: String,
  email: String,
  active: bool,
  logins: u64
}

fn build_user_longform(username: String, email: String) -> User {
  User {
    username: username,
    email: email,
    active: true,
    logins: 1
  }
}

fn build_user_shortform(username: String, email: String) -> User {
  User {
    username,
    email,
    active: true,
    logins: 1
  }
}

fn main() {
  /*
    Values don't have to inputted in the same order as the struct definition 
  */
  let user = User {
    email: String::from("amanda@seyfried.com"),
    username: String::from("amandaseyfried"),
    active: true,
    logins: 32  
  };

  /*
    Use dot notation to retrive fields
  */
  println!("username: {}", user.username);
  println!("email: {}", user.email);
  println!("logins: {}", user.logins);
  println!("active: {}", user.active);

  /*
    //user.username = String::from("jessicachastain");
    Editing a field on a class that's not mutable is not possible.
    For mutability, all fields in the class must be mutable
  */

  let mut user2 = User {
    username: String::from("jessicachastain"),
    email: String::from("jessica@gmail.com"),
    active: true,
    logins: 2
  };

  user2.email = String::from("jessica.chastain@gmail.com");

  /* 
    Can use a constructor function
  */
  let _user3 = build_user_longform(String::from("amandaseyfried"), String::from("amanda.seyfried@gmail.com"));
  let _user4 = build_user_shortform(String::from("amandaseyfried"), String::from("amanda.seyfried@gmail.com"));

  /* 
    Struct update syntax 
  */
  let _user5 = User {
    username: String::from("amandaseyfried"),
    email: String::from("amanda.seyfried@gmail.com"),
    .._user4
  };

  /* 
    Tuple Structs
  */

  struct Color (u8, u8, u8);
  let _white = Color(255, 255, 255);

  struct Point (i64, i64, i64);
  let _planet = Point(149, 19, 20);

}
