// ex03 — Read from stdin, parse to numbers, compute rectangle area
// What this demonstrates:
// - Reading user input via `io::stdin().read_line(...)` into mutable `String`s.
// - Converting input `String` → `u32` with `trim().parse()`.
// - Handling errors: `.expect(...)` for I/O read failures; `match` on `parse()` to
//   print the parse error and exit early if the user enters a non-number.
// - Defining a `Rectangle` and an `impl` block with an `area(&self)` method.
// - Using `#[derive(Debug)]` so `Rectangle` can be printed with `{:?}`.

use std::io;

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 { self.width * self.height }
}

fn main() {
  let mut width = String::new();
  let mut height = String::new();

  println!("Let's calculate area of a rectangle");
  println!("Enter width of the rectangle:");
  io::stdin()
    .read_line(&mut width)
    .expect("Failed to read the entry");
  let width: u32 = match width.trim().parse() {
    Ok(num) => num,
    Err(e) => {
      println!("{e}");
      return;
    },
  };

  println!("Enter height of the rectangle:");
  io::stdin()
    .read_line(&mut height)
    .expect("Failed to read the entry");
  let height: u32 = match height.trim().parse() {
    Ok(num) => num,
    Err(e) => {
      println!("{e}");
      return;
    },
  };

  let rect1 = Rectangle { width: width, height: height };
  println!("The area of the {:?} you have enter is {}", rect1, rect1.area());
}
