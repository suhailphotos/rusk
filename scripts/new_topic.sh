#!/usr/bin/env bash
set -euo pipefail
topic="${1:-}"
[[ -z "$topic" ]] && { echo "usage: scripts/new_topic.sh <topic>"; exit 1; }

crate_name=$(awk -F '"' '/^\s*name\s*=\s*/ {print $2; exit}' Cargo.toml)
mkdir -p "src/$topic" "src/bin"

cat > "src/$topic/mod.rs" <<RS
//! Topic docs. Add doctests with \`\`\`rust blocks.

pub fn demo() {
    println!("demo: $topic");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_runs() {
        assert_eq!(2 + 2, 4);
    }
}
RS

# expose module (append only if missing)
grep -q "pub mod $topic;" src/lib.rs || echo "pub mod $topic;" >> src/lib.rs

# bin wrapper
cat > "src/bin/$topic.rs" <<RS
fn main() { ${crate_name}::$topic::demo(); }
RS

echo "Added topic: $topic"
