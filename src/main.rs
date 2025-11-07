// ex06 — Outer attribute on a `let` binding
// What this shows:
// - You can attach an *outer attribute* directly to a `let` statement.
// - Here we use standard lints from the compiler to keep it simple.
//   * `unused_variables`: allow an unused variable
//   * `non_snake_case`: allow an uppercase variable name (normally warned)
//
// Pattern demonstrated:
// #[some_attribute]
// let PATTERN: Type = initializer_expression;

fn main() {
    #[allow(unused_variables, non_snake_case)]
    let ANSWER: i32 = 42;

    println!("outer attribute on let: OK");
}
