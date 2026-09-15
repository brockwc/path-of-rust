# Path of Rust

CSPB 3112 capstone (Fall 2026). Learning Rust by building a complete, playable roguelike that
runs in the terminal.

## Status

- Sep 15, 2026: project chosen; proposal drafted (course HQ: `~/dev/school/cspb3112`)
- Current phase: Rust ramp-up (Rustlings, The Rust Book ch 1-13, three small CLI programs)
- Game build starts after the ramp-up. Stack decision (ratatui + crossterm vs bracket-lib) is
  recorded here at the end of September.

## Layout

- `crates/path-of-rust`: the game crate (placeholder until the build phase starts)
- `learning/`: ramp-up crates (CLI programs), added as they are built

## Build and test

```
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo run -p path-of-rust
```
