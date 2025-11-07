// ex07 — Platform-specific value via `#[cfg]` on a `let` binding
// What this shows:
// - Using outer attributes `#[cfg(...)]` on `let` to select a value at compile time.
// - Both bindings use the same name and type; only one is compiled.
// - Example chooses the HOME-like environment variable per OS, then reads it.

use std::env;

fn main() {
    #[cfg(target_os = "windows")]
    let home_var: &str = "USERPROFILE";

    #[cfg(not(target_os = "windows"))]
    let home_var: &str = "HOME";

    let val = env::var(home_var).unwrap_or_else(|_| "<unset>".into());
    println!("{home_var} = {val}");
}
