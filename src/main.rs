// ex04 — Custom trait + stdin parsing
// What this shows:
// - Defining a trait (`Area`) with `fn area(&self) -> u32`.
// - Implementing that trait for a concrete type (`Rectangle`) and calling `rect1.area()`.
// - Reading user input via `io::stdin().read_line(...)` into `String`s.
// - Converting input `String` → `u32` with `trim().parse()` and handling errors by printing and exiting early.
// - `#[derive(Debug)]` enables `{:?}` for `Rectangle`.
// Note: This calls the trait method via static dispatch on `Rectangle` (no trait objects).

use std::io;

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

trait Area {
  fn area(&self) -> u32;
}

impl Area for Rectangle {
  fn area(&self) -> u32 { self.width * self.height }
}

fn main() {
  let mut width: String = String::new();
  let mut height: String = String::new();

  println!("Let's build a rectangle!");

  println!("Please enter its width:");
  io::stdin()
    .read_line(&mut width).expect("Failed to read input!");
  let width: u32 = match width.trim().parse() {
    Ok(num) => num,
    Err(e) => {
      println!("{e}");
      return;
    },
  };

  println!("Please enter its height:");
  io::stdin()
    .read_line(&mut height).expect("Failed to read input!");
  let height: u32 = match height.trim().parse() {
    Ok(num) => num,
    Err(e) => {
      println!("{e}");
      return;
    },
  };

  let rect1 = Rectangle { width: width, height: height };
  println!("Here is a {:?}, with area {}", rect1, rect1.area());
}
