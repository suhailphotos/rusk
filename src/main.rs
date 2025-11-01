// ex02 — Deriving `Debug` + simple method (`area`)
// Purpose: show that `#[derive(Debug)]` enables `{:?}` without manual impls.
//
// Key notes:
// - `#[derive(Debug)]` auto-implements `std::fmt::Debug` for `Rectangle`.
//   This works because all fields (`u32`) already implement `Debug`.
// - `{:#?}` would pretty-print; `{}` would NOT work unless you implement `Display`.

#[derive(Debug)] // enables `println!("{:?}", rect1)` → Rectangle { width: 30, height: 50 }
struct Rectangle {
  width: u32,  // rectangle width (unsigned 32-bit)
  height: u32, // rectangle height (unsigned 32-bit)
}

impl Rectangle {
  // `&self` is an immutable borrow; we can read fields but not mutate.
  // Returns area as `u32` (fits for these small values; in real apps consider overflow).
  fn area(&self) -> u32 {
    self.width * self.height
  }
}

fn main() {
  // Construct a Rectangle literal by naming fields.
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  // `{:?}` → uses the derived Debug impl for Rectangle.
  // `\n` adds a newline. `{}` → uses Display for `u32` (built-in).
  println!("Here is {:?}\nAnd its area is {}", rect1, rect1.area());
  // Example output:
  // Here is Rectangle { width: 30, height: 50 }
  // And its area is 1500
}
