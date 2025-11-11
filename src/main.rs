// Fixing an Unsafe Program: Aliasing and Mutating a Data Structure
//
// Original idea (does NOT compile and would be unsafe if it did):
//
// fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
//     // Find a reference to the largest String in `dst`.
//     let largest: &String = dst.iter()
//         .max_by_key(|s| s.len())
//         .unwrap();
//
//     // ❌ Problem: `largest` is a reference into `dst`, but we also mutate `dst`
//     // via `push`. A push may reallocate the Vec's buffer and move its contents,
//     // which would invalidate `largest`.
//     for s in src {
//         if s.len() > largest.len() {
//             dst.push(s.clone());
//         }
//     }
// }
//
// The borrow checker rejects this because the immutable borrow (`largest`)
// overlaps with a mutation of `dst` (`dst.push(..)`).

// ──────────────────────────────────────────────────────────────────────────────
// Fix 1: Clone the largest String itself.
//
// We store an owned `String` instead of a `&String`, so the borrow of `dst`
// ends as soon as `largest` is computed. The subsequent `push` no longer
// overlaps with a borrow.
fn add_big_strings_clone_largest(dst: &mut Vec<String>, src: &[String]) {
    let largest: String = dst
        .iter()
        .max_by_key(|s| s.len())
        .unwrap()
        .clone(); // take ownership of a copy

    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Fix 2: Collect the items to add into a temporary Vec, then extend `dst`.
//
// We keep a reference (`&String`) only while computing `to_add`. Once `to_add`
// is built, the borrow of `dst` ends, and only then do we mutate `dst`.
fn add_big_strings_collect(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = dst
        .iter()
        .max_by_key(|s| s.len())
        .unwrap();

    let to_add: Vec<String> = src
        .iter()
        .filter(|s| s.len() > largest.len())
        .cloned()
        .collect();

    dst.extend(to_add);
}

// ──────────────────────────────────────────────────────────────────────────────
// Fix 3 (most idiomatic / performant): only keep the length.
//
// We don't actually need a reference to the largest string, just its length.
// By copying out `usize`, we completely avoid a long-lived borrow of `dst`
// and can freely push while using `largest_len`.
fn add_big_strings_len(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst
        .iter()
        .max_by_key(|s| s.len())
        .unwrap()
        .len();

    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}

fn main() {
    // Base data: a vector of words (dst) and a source slice (src).
    let base: Vec<String> = [
        "Lorem", "Ipsum", "is", "simply", "dummy",
        "text", "of", "the", "printing",
        "and", "typesetting", "industry.",
    ]
    .iter()
    .map(|&w| w.to_string())
    .collect();

    let src: Vec<String> = vec!["Where does it come from?".to_string()];

    // Use three separate dst vectors so we can see all variants.
    let mut dst_clone   = base.clone();
    let mut dst_collect = base.clone();
    let mut dst_len     = base.clone();

    add_big_strings_clone_largest(&mut dst_clone, &src);
    add_big_strings_collect(&mut dst_collect, &src);
    add_big_strings_len(&mut dst_len, &src);

    println!("clone_largest : {:?}", dst_clone);
    println!("collect+extend: {:?}", dst_collect);
    println!("len-only      : {:?}", dst_len);
}
