//! String slices & lifetimes — minimal demos and tests.
//!
//! ## Quick doctest
//! ```rust
//! use rusk::strslice::echo;
//! assert_eq!(echo("hello"), "hello");
//! ```

/// Idiomatic (lifetime elided)
pub fn echo(s: &str) -> &str { s }

/// Explicit lifetime (exactly equivalent to `echo`)
pub fn echo_explicit<'a>(s: &'a str) -> &'a str { s }

/// Show returning one of two inputs (output ties to `'a`)
pub fn pick_first<'a>(a: &'a str, _b: &str) -> &'a str { a }

/// Safe helper: returns length (avoids tricky UTF-8 slicing in examples)
pub fn len(s: &str) -> usize { s.len() }

pub fn demo() {
    let hi = "hi";
    println!("echo         -> {}", echo(hi));
    println!("echo_explicit-> {}", echo_explicit(hi));
    println!("len('rusk')  -> {}", len("rusk"));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn echo_roundtrips() { assert_eq!(echo("ok"), "ok"); }

    #[test]
    fn pick_first_works() {
        let a = "AAA";
        let b = "BBB";
        let r = pick_first(a, b);
        assert_eq!(r, "AAA");
    }
}
