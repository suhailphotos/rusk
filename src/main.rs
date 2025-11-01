struct Rectangle {
  width: u32,
  height: u32,
}

impl std::fmt::Debug for Rectangle {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Its a rectangle with width: {} & height: {}", self.width, self.height)
  }
}

fn main() {
  let rect1 = Rectangle {
    width: 32,
    height: 93,
  };

  println!("{rect1:?}");
}
