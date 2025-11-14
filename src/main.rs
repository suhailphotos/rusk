fn main() {
  let x: Box<i32> = Box::new(0);      // x: Box<i32>
  let y: Box<&Box<i32>> = Box::new(&x);     // y: Box<&Box<i32>>
  let z: i32 = ***y;
  let a: &Box<i32> = *y;
//let b: Box<i32> = **y;
  println!("{a}");// z: i32
//println!("{b}");
  println!("{z}");
}
