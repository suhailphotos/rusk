use std::collections::BTreeMap;

fn main() {
    let vars: BTreeMap<String, String> = std::env::vars().collect();
    println!("{vars:#?}");
}

