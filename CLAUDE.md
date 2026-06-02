# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

A Cargo workspace of standalone Rust solutions to [Project Euler](https://projecteuler.net/) problems. Each problem lives in its own crate under `solutions/pe-NNNN/` and produces a binary that prints the answer and elapsed time. The shared `pe-utils` crate (at the repo root) provides the timing harness.

## Commands

```bash
# Build a single problem
cargo build -p pe-0001

# Run a single problem (from repo root; some solutions read relative data/ paths)
cargo run -p pe-0001

# Run tests for a problem that has them
cargo test -p pe-0081

# Build all solutions and report failures
./build-all.sh

# Build the entire workspace
cargo build --workspace
```

> Solutions that read data files (e.g. `pe-0013`, `pe-0022`, `pe-0042`, `pe-0081`) use relative paths like `data/filename.txt`. Run them from the crate directory (`cd solutions/pe-0081 && cargo run`) or they will fail to open the file.

## Architecture

### `pe-utils` (shared library)

`pe-utils/src/lib.rs` provides two things:

- `pe_utils::run(problem_num, solve_fn)` — calls `solve_fn`, prints the problem number, answer, and elapsed milliseconds.
- `pe_utils::pe_main!()` — a macro that generates `fn main()`. It derives the problem number from the crate name at compile time (`pe-0042` → 42) and calls `pe_utils::run(42, solve)`.

### Solution crate pattern

Every solution follows this structure:

```rust
// Problem title
// https://projecteuler.net/problem=N

fn solve() -> <ReturnType> {
    // ...
}

pe_utils::pe_main!();
```

The `solve()` function must return a `Display` type. `pe_main!()` must appear after `solve()` is defined.

### Workspace dependencies

All external crates are declared once in the root `Cargo.toml` under `[workspace.dependencies]` and referenced in crate manifests with `{ workspace = true }`. Available: `pe-utils`, `num-bigint`, `num-traits`, `num`, `num-rational`, `itertools`, `primal`, `primes`, `csv`, `chrono`.

### Adding a new problem

1. `cargo new solutions/pe-NNNN --name pe-NNNN` (crate name must match `pe-NNNN` for the macro to extract the problem number).
2. Add `"solutions/pe-NNNN"` to the `members` list in the root `Cargo.toml`.
3. In `solutions/pe-NNNN/Cargo.toml`, add `pe-utils = { workspace = true }` and any needed workspace deps.
4. Write `fn solve()` and call `pe_utils::pe_main!();` at the bottom of `src/main.rs`.
5. If the problem needs a data file, place it under `solutions/pe-NNNN/data/` and read it with a relative path.
