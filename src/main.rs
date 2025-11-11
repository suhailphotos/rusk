// Fixing an Unsafe Program: Copying vs. Moving Out of a Collection
//
// When you read a value out of a collection, you have to be clear on whether
// you are *copying* it or *moving* it. For `Copy` types like `i32`, a simple
// read is fine. For non-`Copy` types like `String`, trying to move through a
// shared reference is rejected by the compiler to prevent double-free bugs.

// ──────────────────────────────────────────────────────────────────────────────
// 1) Safe: copying an i32 out of a Vec<i32>.
//
// `i32` implements `Copy`, so reading through a shared reference just copies
// the bits. No ownership is moved out of the Vec.
fn copy_i32_from_vec() {
    let v: Vec<i32> = vec![0, 1, 2];
    let n_ref: &i32 = &v[0]; // shared reference into the vector
    let n: i32 = *n_ref;     // copies the i32 value
    println!("copy_i32_from_vec: v = {:?}, n = {}", v, n);
}

// ──────────────────────────────────────────────────────────────────────────────
// 2) Illegal (conceptually): moving a String out through a shared reference.
//
// This is what the book shows *not* compiling:
//
// fn move_string_through_ref() {
//     let v: Vec<String> = vec![String::from("Hello world")];
//     let s_ref: &String = &v[0];
//     let s: String = *s_ref; // ❌ cannot move out of `*s_ref` behind `&`
// }
//
// If this were allowed, both `v` and `s` would think they own the same String,
// causing a double-free when both are dropped.

// ──────────────────────────────────────────────────────────────────────────────
// 3) Safe pattern A: just borrow the element as &String.
//
// We never take ownership of the String; we only look at it via a reference.
fn borrow_string_from_vec() {
    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    println!("borrow_string_from_vec: {s_ref}!");
    // v is still fully usable here.
}

// ──────────────────────────────────────────────────────────────────────────────
// 4) Safe pattern B (your example): clone the String.
//
// We explicitly clone the element so that we get our own owned `String`.
// The Vec keeps its original element; no move happens through the reference.
fn clone_string_from_vec() {
    let v: Vec<String> = vec![String::from("Hello, World")];
    let mut s: String = v[0].clone(); // allocate and copy the contents
    s.push_str("!");
    println!("clone_string_from_vec: s = {s}, v = {:?}", v);
}

// ──────────────────────────────────────────────────────────────────────────────
// 5) Safe pattern C: move the String out of the Vec with `remove`.
//
// Here we *do* move the element, but we do it through the Vec API itself.
// After `remove`, that element no longer lives in the Vec; ownership is now
// entirely in `s`.
fn remove_string_from_vec() {
    let mut v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v.remove(0); // moves the String out of the Vec
    s.push('!');
    println!("remove_string_from_vec: s = {s}, v = {:?}", v);
    assert!(v.len() == 0);
}

// ──────────────────────────────────────────────────────────────────────────────

fn main() {
    copy_i32_from_vec();
    borrow_string_from_vec();
    clone_string_from_vec();
    remove_string_from_vec();
}

