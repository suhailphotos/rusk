#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 { self.width * self.height }
}

fn main() {
  let r = Rectangle { width: 32, height: 34 };
  println!("Its a {:?}, with area {}", r, r.area());
}
