// Fixing an Unsafe Program: Returning a Reference to the Stack
//
// Original (unsafe / does not compile):
//
// fn return_a_string() -> &String {
//     let s = String::from("Hello world");
//     &s  // ❌ returns a reference to a local (stack) value
// }
//
// The fixes below all avoid returning a reference to a local `String`.
// Each one chooses a different ownership / lifetime strategy.

use std::rc::Rc;

// 1) Move the owned `String` out of the function.
//    Caller now owns and is responsible for deallocation.
fn return_a_string_fix_a() -> String {
    let s: String = String::from("Hello, world!");
    s
}

// 2) Return a string literal with a `'static` lifetime.
//    The data is embedded in the binary and lives for the entire program.
fn return_a_string_fix_b() -> &'static str {
    "Hello, world!"
}

// 3) Use reference counting (`Rc<String>`).
//    Ownership is shared; deallocation happens when the last `Rc` is dropped.
fn return_a_string_fix_c() -> Rc<String> {
    let s = Rc::new(String::from("Hello, world!"));
    Rc::clone(&s)
}

// 4) Caller supplies a "slot" via `&mut String`.
//    The function writes into the existing buffer instead of returning anything.
fn return_a_string_fix_d(output: &mut String) {
    // Replace the entire contents of `output` with the new text.
    output.replace_range(.., "Hello, world!");
}

fn main() {
    println!("{}", return_a_string_fix_a());
    println!("{}", return_a_string_fix_b());
    println!("{}", return_a_string_fix_c());

    // For the `&mut String` version, we:
    // 1. Create an empty String we own,
    // 2. Pass a mutable reference so the function can fill it,
    // 3. Print the mutated value.
    let mut s: String = String::new();
    return_a_string_fix_d(&mut s);
    println!("{s}");
}
