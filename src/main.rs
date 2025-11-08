use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
  // Demo setup: create a temporary file with known contents ("42")
  // so we have a predictable Ok(...) value to work with.
  let mut path: PathBuf = std::env::temp_dir();
  path.push("rust_result_demo.txt");

  fs::write(&path, "42")?;

  // --------------------------------------------------
  // Reading a Result with `unwrap` and `expect`
  // --------------------------------------------------
  //
  // Pattern:
  //   let value = result.unwrap();
  //   let value = result.expect("error message");
  //
  // - On Ok(T), returns the inner value T.
  // - On Err(E), panics:
  //     * `unwrap()` uses a generic panic message.
  //     * `expect()` uses your custom error message.
  // - Best suited for quick demos, tests, or truly unrecoverable errors.
  let s1 = fs::read_to_string(&path).unwrap();
  let _ = fs::read_to_string(&path).expect("failed to read file");
  println!("A) unwrap: s1 = {:?}", s1);


  // --------------------------------------------------
  // Providing a fallback with `unwrap_or` / `unwrap_or_else`
  // --------------------------------------------------
  //
  // Pattern:
  //   let value = result.unwrap_or(default_value);
  //   let value = result.unwrap_or_else(|err| compute_from(err));
  //
  // - Both convert Result<T, E> into T by replacing Err(E) with a fallback.
  // - `unwrap_or`:
  //     * Takes a T value directly (eagerly evaluated).
  // - `unwrap_or_else`:
  //     * Takes a closure that receives the error E and computes a T (lazy).
  let missing_path = path.with_file_name("this_file_does_not_exists.txt");
  let s2 = fs::read_to_string(&missing_path).unwrap_or(String::from("fallback"));
  let s3 = fs::read_to_string(&missing_path).unwrap_or_else(|_e| "computed fallback".to_string());
  println!("B) unwrap_or: {:?}, unwrap_or_else: {:?}", s2, s3);

  // --------------------------------------------------
  // Handling Result with a `match` expression
  // --------------------------------------------------
  //
  // Pattern:
  //   let value = match expression_returning_result {
  //       Ok(v) => v,         // success path, returns T
  //       Err(e) => { ... }   // error path, must also produce a T
  //   };
  //
  // - Gives you full control over both success and error cases.
  // - Both arms must return the same type T.
  let s4 = match fs::read_to_string(&path) {
    Ok(text) => text,
    Err(e) => {
      eprintln!("C) read error: {e}");
      String::new()
    }
  };
  println!("C) match: s4 = {:?}", s4);

  // --------------------------------------------------
  // Reading a Result with `if let`
  // --------------------------------------------------
  //
  // Pattern:
  //   if let Ok(value) = expression_returning_result {
  //       // runs only on success; `value` is the inner T
  //   } else {
  //       // runs on error; `value` is NOT in scope here
  //   }
  //
  // - A concise way to handle only the Ok branch explicitly.
  // - Common when the Err branch just logs or does simple fallback work.
  // - Here we only care about the side effects (printing), so the
  //   value of the `if let` expression itself is ignored.
  if let Ok(text) = fs::read_to_string(&path) {
    println!("D) if let Ok: {}", text.trim());
  } else {
    println!("D) if let Ok: fall back");
  }

  // --------------------------------------------------
  // Propagating errors with the `?` operator
  // --------------------------------------------------
  //
  // Pattern:
  //   let value = expression_returning_result?;
  //
  // - On Ok(T), yields T.
  // - On Err(E), returns Err(From::from(E)) from the current function.
  // - The enclosing function must itself return a Result<_, _>.
  // - Great for “bubbling up” errors and keeping code linear.
  let s5 = fs::read_to_string(&path)?;
  println!("E) `?` operator: s5 = {:?}", s5);

  // --------------------------------------------------
  // Converting Result<T, E> to Option<T> with `.ok()`
  // --------------------------------------------------
  //
  // Pattern:
  //   let maybe = result.ok();              // Result<T, E> -> Option<T>
  //   let value = maybe.unwrap_or_default(); // Option<T> -> T using T::default()
  //
  // - `.ok()` discards the error and keeps only the presence/absence of T.
  // - `unwrap_or_default()`:
  //     * On Some(t), returns t.
  //     * On None, returns T::default().
  let maybe_s = fs::read_to_string(&missing_path).ok();
  let s6 = maybe_s.unwrap_or_default();
  println!("F) ok() + unwrap_or_default(): {:?}", s6);

  // --------------------------------------------------
  // Inspecting error details and boxing as `Box<dyn Error>`
  // --------------------------------------------------
  //
  // Pattern:
  //   let res = expression_returning_result;
  //   match res {
  //       Ok(value) => { ... }
  //       Err(e) => {
  //           // inspect error (e.g. e.kind())
  //           let boxed: Box<dyn Error> = e.into();
  //       }
  //   }
  //
  // - `io::Error::kind()` lets you branch on specific I/O error kinds.
  // - Converting to `Box<dyn Error>` is useful for returning
  //   heterogeneous errors via a common trait object.
  let res = OpenOptions::new().write(true).create_new(true).open(&path);
  match res {
    Ok(_fh) => println!("G) unexpectedly created a new file"),
    Err(e) => {
      match e.kind() {
        io::ErrorKind::AlreadyExists => println!("G) error kind = AlreadyExists (as expected)"),
        io::ErrorKind::PermissionDenied => {
          println!("G) error kind = PermissionDenied (e.g., locked on some platforms)")
        }
        other => println!("G) error kind = {:?}", other),
      }

      let boxed: Box<dyn Error> = e.into();
      println!("G) boxed error: {}", boxed);
    }
  }

  Ok(())
}
