# rusk

Hands-on Rust topic playground with runnable demos & tests. Each topic is a small module with a `demo()` function, unit tests, and doctests. Run any topic as a separate binary.

## Quick start

```bash
# list runnable topics (bins live under src/bin)
ls src/bin

# run a topic
cargo run --bin structs
cargo run --bin strslice

# run all tests (includes doctests)
cargo test

# build & view docs
cargo doc --open
```

## Topics (initial)

- `structs` — named/tuple/unit-like, methods vs free functions, Debug.
- `strslice` — `&str` basics, literals, borrowing from `String`, echo lifetime example.

> Add more with `scripts/new_topic.sh <topic>` (e.g., `enums`, `traits`, `lifetimes`).
> Prefer short, all-lowercase names (e.g., `iter`, `traits`, `lifetm`).

## Layout

```
src/
  lib.rs          # exposes topics: pub mod structs; pub mod strslice;
  structs/
    mod.rs        # topic module (demo + tests + docs)
  strslice/
    mod.rs
  bin/
    structs.rs    # calls rusk::structs::demo()
    strslice.rs   # calls rusk::strslice::demo()
scripts/
  new_topic.sh    # scaffold a new topic: modules + bin wrapper
.github/workflows/
  ci.yml          # CI: fmt, clippy, tests
```

## Adding a new topic

```bash
scripts/new_topic.sh enums
cargo run --bin enums
```

The script creates:
- `src/enums/mod.rs` with a `demo()` and a starter test
- `pub mod enums;` appended to `src/lib.rs` if missing
- `src/bin/enums.rs` that calls `rusk::enums::demo()`

## Conventions

- **Names**: short, all-lowercase (avoid underscores unless really needed).
- **Demos**: put runnable examples in `demo()`.
- **Doctests**: tiny facts in `///` or `//!` doc blocks with ```rust snippets.
- **Tests**: quick unit tests in each topic’s `#[cfg(test)]` module.

## CI

GitHub Actions runs on push and PR:
- `cargo fmt --check`
- `cargo clippy -D warnings`
- `cargo test`

## License

MIT
