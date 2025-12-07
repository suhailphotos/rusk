fn main() {
    let mut _x: i32 = 5;

    let _r: &mut i32 = &mut _x;

    *_r += 1;
    *_r -= 3;

    println!("Value of _r : {}", _r);
    println!("Value of _x : {}", _x);
}
