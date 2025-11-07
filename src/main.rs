// Conditional variable assignment using an `if` expression.
//
// General pattern:
// let x: T = if condition {
//     value_if_true
// } else {
//     value_if_false
// };
//
// `T` (the explicit type annotation) is optional if the compiler can infer
// the type from the values of both branches.

fn main() {
    let n = 7;

    // With explicit type annotation:
    let description: &str = if n % 2 == 0 {
        "even"
    } else {
        "odd"
    };

    // The same pattern with type inference (no explicit `T`):
    let sign = if n >= 0 { "non-negative" } else { "negative" };

    println!("n = {n}, description = {description}, sign = {sign}");
}
