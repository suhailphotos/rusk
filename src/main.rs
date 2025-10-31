use std::io;

fn main() {
  println!("Let's play a guessing game.\nGuess a number:");

  let mut guess = String::new();

  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read the input");

  let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(e) => {
      println!("Please enter a valid number. Error: {e}");
      return;
    },
  };

  println!("You guessed {guess}");
}
