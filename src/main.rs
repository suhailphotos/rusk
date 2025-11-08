fn main() {
  let text = "loren ipsum dolor sit amet";

  let pos: Option<usize> = text.find('i');

  // --------------------------------------------------
  // Unwrapping Option with a fallback:
  //   - `unwrap_or(default_value)`
  //   - `unwrap_or_else(|| compute_default())`
  // --------------------------------------------------
  //
  // Pattern:
  //   let value = opt.unwrap_or(fallback);
  //   let value = opt.unwrap_or_else(|| fallback_computation());
  //
  // - On Some(v), both return v.
  // - On None:
  //     * `unwrap_or` returns the eagerly-provided fallback value.
  //     * `unwrap_or_else` calls the closure to compute a fallback.
  let idx1: usize = pos.unwrap_or(0);
  let idx2: usize = pos.unwrap_or_else(|| expensive_default_index());
  println!("A) idx1 = {idx1}, idx2 = {idx2}");

  // --------------------------------------------------
  // Handling Option with `match`
  // --------------------------------------------------
  //
  // Pattern:
  //   let value = match opt {
  //       Some(v) => v,
  //       None => fallback,
  //   };
  //
  // - Explicitly handles both Some and None.
  // - Both arms must produce the same type.
  let idx3: usize = match pos {
    Some(i) => i,
    None => 0,
  };
  println!("B) idx3 (match) = {idx3}");

  // --------------------------------------------------
  // Pattern-matching Option with `if let`
  // --------------------------------------------------
  //
  // Pattern:
  //   if let Some(v) = opt {
  //       // runs only when opt is Some(v)
  //   } else {
  //       // runs when opt is None; `v` not in scope here
  //   }
  //
  // - A concise way to handle the Some case and do something simple
  //   in the None branch (like logging or fallback).
  if let Some(i) = pos {
    println!("C) found at index {i}");
  } else {
    println!("C) not found, using fallback");
  }

  // --------------------------------------------------
  // Using `?` in a function that returns Option<T>
  // --------------------------------------------------
  //
  // Pattern (inside an Option-returning function):
  //   let v = expression_returning_option?;
  //   // `v` is the inner T on Some(T),
  //   // or the whole function returns None on None.
  //
  // - `?` short-circuits with None, instead of an error.
  // - Keeps Option-based control flow linear and easy to read.
  let doubled = find_and_double(text);
  println!("D) doubled index via `?` in Option-returning fn = {:?}", doubled);

  // --------------------------------------------------
  // Converting Option<T> to Result<T, E> with `.ok_or(...)`
  // --------------------------------------------------
  //
  // Pattern:
  //   let res: Result<T, E> = opt.ok_or(err_value);
  //
  // - Some(v) -> Ok(v)
  // - None    -> Err(err_value)
  //
  // Useful when an "absence of value" should become a proper error.
  let idx_res: Result<usize, &'static str> = text.find('x').ok_or("char \"x\" not found");
  match idx_res {
    Ok(i) => println!("E) Ok -> index = {i}"),
    Err(e) => println!("E) Err -> {e}"),
  }

  // --------------------------------------------------
  // Transforming the inner value of Option with `.map(...)`
  // --------------------------------------------------
  //
  // Pattern:
  //   let mapped: Option<U> = opt.map(|v| transform(v));
  //
  // - Some(v) -> Some(transform(v))
  // - None    -> None
  //
  // Here:
  //   - If `find('i')` returns Some(i), we slice from that index onward.
  //   - If it returns None, the whole expression is None.
  let after_i: Option<&str> = text.find('i').map(|i| &text[i..]);
  println!("F) slice from 'i': {:?}", after_i);
}

// Example "expensive" fallback function used with `unwrap_or_else`.
fn expensive_default_index() -> usize { 0 }

// Demonstrates `?` in an Option-returning function:
// - If `find('i')` returns Some(i), we return Some(i * 2).
// - If it returns None, `?` makes the whole function return None.
fn find_and_double(s: &str) -> Option<usize> {
  let i = s.find('i')?;
  Some(i * 2)
}
