//! &str basics, literals, borrowing from String, echo lifetime.

pub fn echo<'a>(s: &'a str) -> &'a str { s }

pub fn demo() {
    let lit: &'static str = "hello";
    let owned = String::from("world");
    let sl: &str = &owned; // or owned.as_str()
    println!("lit: {}", lit);
    println!("slice: {}", sl);
    println!("echo(lit): {}", echo(lit));
    println!("echo(slice): {}", echo(sl));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn echo_roundtrips() {
        assert_eq!(echo("abc"), "abc");
        let s = String::from("xyz");
        assert_eq!(echo(&s), "xyz");
    }
}
