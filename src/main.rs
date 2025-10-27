use std::io;
use rand::Rng;

fn main(){
  let mut guess = String::new();
  let sec_num = rand::rng().random_range(1..=100);

  println!("Pick a number from 1 to 100");
  println!("What's your pick:");

  io::stdin()
    .read_line(&mut guess)
    .expect("You didn't do what I asked you to do, try again");

  println!("You picked: {guess}");
  println!("And I picked {sec_num}");

}
