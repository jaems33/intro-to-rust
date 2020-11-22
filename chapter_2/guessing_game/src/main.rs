use std::io;
use std::cmp::Ordering;
use rand::{thread_rng, Rng};

fn main() {
  println!("Guess the number!");

  let secret_number = thread_rng().gen_range(0, 101);
  
  loop {
    let mut guess = String::new();
    println!("Input your guess:");
    
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line.");

    let guess: u32 = match guess.trim().parse(){
      Ok(num) => num,
      Err(_) => continue,
    };

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
          println!("You win!");
          break;
        }
    }
  }
}
