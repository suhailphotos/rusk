// Manual `Debug` for a simple struct
// Purpose: show how `{:?}` dispatches to `std::fmt::Debug::fmt`
// Commit (frozen reference):
//
// Key notes:
// - Signature: `fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`
//   * `self` is read-only; `f` is mutable because we write into the buffer.
// - `write!(f, ...)` returns `fmt::Result`; keep it as the tail expression (no semicolon).
// - `{:?}` uses `Debug`; `{}` would require implementing `Display`.
//
// Expected run:
//   cargo run
//   → Its a rectangle with width: 32 & height: 93

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
