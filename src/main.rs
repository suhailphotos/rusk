// Fixing an Unsafe Program: Not Enough Permissions
//
// Original (does not compile):
//
// fn stringify_name_with_title(name: &Vec<String>) -> String {
//     // ❌ `name` is an immutable reference (`&Vec<String>`), so we only have read permission.
//     //    `push` requires write permission (`W`), so the borrow checker rejects this.
//     name.push(String::from("Esq."));
//     let full = name.join(" ");
//     full
// }
//
// If this were allowed, callers could observe surprising mutations or, worse,
// have their existing references invalidated when `push` reallocates the Vec.
//
// Below are several *compiling* variants that each encode a different API choice.

// 1) Mutate the input via &mut Vec<String>.
//    This is legal, but semantically questionable for a function that sounds “read-only”.
fn stringify_name_with_title_mutating(name: &mut Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}

// 2) Take ownership of the Vec<String>.
//    Also legal, but often annoying to callers because they lose their Vec.
fn stringify_name_with_title_taking_ownership(mut name: Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    // `name` is dropped here; caller cannot use it again.
    full
}

// 3) Keep the original &Vec<String> API and clone the vector.
//    Safe and explicit, but may copy more than needed for large inputs.
fn stringify_name_with_title_clone(name: &Vec<String>) -> String {
    let mut name_clone = name.clone();          // clone the Vec and all its Strings
    name_clone.push(String::from("Esq."));      // mutate the local clone freely
    let full = name_clone.join(" ");
    full
}

// 4) Keep &Vec<String>, but avoid cloning the entire Vec.
//    `join` already copies all strings into a new `String`, so we just append the suffix.
fn stringify_name_with_title_suffix(name: &Vec<String>) -> String {
    let mut full = name.join(" ");              // copies the pieces into one String
    full.push_str(" Esq.");                     // append suffix to the result
    full
}

fn main() {
    // Demonstrate clone/suffix versions: input is not mutated and stays usable.
    let name = vec![String::from("Ferris"), String::from("Jr.")];

    let full_clone   = stringify_name_with_title_clone(&name);
    let full_suffix  = stringify_name_with_title_suffix(&name);

    println!("original name vec      : {:?}", name);
    println!("clone() version        : {full_clone}");
    println!("join + suffix version  : {full_suffix}");

    // Demonstrate the &mut Vec<T> version: caller *does* observe mutation.
    let mut name2 = vec![String::from("Ferris")];
    let full_mut = stringify_name_with_title_mutating(&mut name2);
    println!("mutating version       : {full_mut}");
    println!("name2 after mutation   : {:?}", name2);

    // Demonstrate the ownership-taking version:
    let name3 = vec![String::from("Ferris")];
    let full_owned = stringify_name_with_title_taking_ownership(name3);
    println!("taking-ownership version: {full_owned}");
    // `name3` cannot be used here; ownership was moved into the function.
}
