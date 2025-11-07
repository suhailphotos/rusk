// ex08 — `let-else` with Option<T> (no turbofish)
// What this shows:
// - Use a type annotation on the binding to guide `parse()` instead of `::<T>`.
// - Convert `Result<T, _>` → `Option<T>` via `.ok()`.
// - Destructure with `let Some(x) = ... else { ... };` (else must diverge).

fn main() {
    let input = "42";
    let maybe_value: Option<u32> = input.parse().ok();

    let Some(x) = maybe_value else {
        panic!("expected a number, got None");
    };

    println!("Parsed value: {x}");
}
