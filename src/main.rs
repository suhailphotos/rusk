// ex16: `&self`, method-call sugar, and when `&` is implied
//
// - `fn area(&self)` is sugar for `fn area(self: &Rectangle)`.
// - `rect1.area()` desugars to `Rectangle::area(&rect1)` (auto-borrow).
// - `(&rect1).area()` also works; parentheses are needed so `&` applies to
//   `rect1` instead of the result of `.area()`.
// - Inside the method, `self` is `&Rectangle`, but field/ops auto-deref/auto-ref,
//   so the commented expressions all compile; the last line is the idiomatic one.

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  // fn area(self: &Rectangle) -> u32 {  // <--- explicit desugared form
  fn area(&self) -> u32 {
    // (&self).width * &self.height      // <--- compiles (auto-deref / auto-ref)
    // &self.width * self.height         // <--- also compiles
    // self.width * &self.height         // <--- also compiles
    // (&self).width * (&self.height)    // <--- also compiles
    self.width * self.height            // <--- idiomatic version
  }
}

fn main() {
  let rect1: Rectangle = Rectangle { width: 32, height: 84 };

  // let area1: u32 = (&rect1).area();
  // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  // Explicit reference to the receiver. Parentheses are required:
  //   (&rect1).area()
  // not:
  //   &rect1.area()   // that would borrow the *result* of area()

  let area1: u32 = rect1.area(); // method-call sugar: Rectangle::area(&rect1)

  println!("The area of {:?} is {}", rect1, area1 );
  //                    ^
  //                    |
  // `{:?}` uses the Debug formatter; #[derive(Debug)] auto-generates the
  // `std::fmt::Debug` impl so this print works.
}
