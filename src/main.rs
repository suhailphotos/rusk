fn main() {
    let x = Box::new(0);      // x: Box<i32>
    let y = Box::new(&x);     // y: Box<&Box<i32>>
    let z: i32 = ***y;        // z: i32
    println!("{z}");
}
