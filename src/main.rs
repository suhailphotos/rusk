use std::io;

fn main() {
  println!("Let's play a guessing game.\nGuess a number:");
  let mut guess = String::new();

  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read the input!");

  let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(e) => {
      println!("I ran into this error {e}");
      return;
    },
  };
  println!("The number you gussed is {guess}")
}
