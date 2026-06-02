# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

A Cargo workspace of standalone Rust solutions to [Project Euler](https://projecteuler.net/) problems. Each problem lives in its own crate under `solutions/pe-NNNN/` and produces a binary that prints the answer and elapsed time. The shared `pe-utils` crate (at the repo root) provides the timing harness.

## Commands

```bash
# Build / run / test a single problem
cargo build -p pe-0001
cargo run   -p pe-0001
cargo test  -p pe-0081

# Build or test all 80+ solutions and report failures
./build-all.sh
./test-all.sh

# Build the entire workspace
cargo build --workspace
```

> Solutions that read data files (e.g. `pe-0013`, `pe-0022`, `pe-0042`, `pe-0081`) use relative paths like `data/filename.txt`. Run them from the crate directory (`cd solutions/pe-0081 && cargo run`) or they will fail to open the file.

## Scripts

| Script | Purpose |
|--------|---------|
| `./new-solution.sh <N>` | Scaffold a new `solutions/pe-NNNN/` crate and add it to the workspace |
| `./build-all.sh` | Build every solution; report failures |
| `./test-all.sh` | Test every solution; report failures |
| `./fetch-problems.sh --all` | Full scrape of projecteuler.net/archives → `problems.toml` (run once) |
| `./fetch-problems.sh` | Incremental update via projecteuler.net/recent → `problems.toml` |

### `problems.toml`

Tracks every published PE problem with title, statement, optional data fields, and a `solved` flag:

```toml
[54]
title = "Poker Hands"
statement = '''
In the card game poker ...'''
data_url = "https://projecteuler.net/resources/documents/0054_poker.txt"
solved = false
```

`data_url` is present for the ~20 problems that ship with a downloadable input file. PE consistently names these files `NNNN_name.txt`. `new-solution.sh` reads this file to insert the problem title in `src/main.rs` and auto-downloads the data file into `data/NNNN_name.txt` if `data_url` is set and the file isn't already there.

`data_embedded = true` marks problems whose input data is embedded directly in the problem statement (e.g. a grid or triangle of numbers) rather than hosted as a separate download. PE provides no `data_url` for these, but the data is still separable from the narrative — the existing solutions for these problems (11, 18, 345) hardcode the data in `src/main.rs`.

`fetch-problems.sh` scrapes titles from `/archives`, statements and data URLs from `/minimal=N` (only re-fetching the ~20 problems whose statement mentions a `.txt`/`.csv` file). Re-running preserves manually set `solved` values; the flag is auto-detected for new entries (directory exists and `src/main.rs` contains no `todo!()`).

Mark a problem solved by editing `problems.toml` directly.

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

Use the scaffolding script — it handles directory creation, boilerplate, and workspace registration in one step:

```bash
./new-solution.sh 42
```

This creates `solutions/pe-0042/` with a `Cargo.toml` and a `src/main.rs` pre-filled with the problem title (from `problems.toml`) and URL, then inserts `"solutions/pe-0042"` into the workspace `members` list in sorted order.

If the problem has a `data_url` in `problems.toml`, the scaffolding script downloads it automatically to `solutions/pe-NNNN/data/NNNN_name.txt`. Read it from `src/main.rs` using a relative path (e.g. `"data/0054_poker.txt"`). For problems without a `data_url`, place any hand-crafted data files in the same `data/` directory.

To add an external dependency, declare it once in the root `Cargo.toml` under `[workspace.dependencies]`, then reference it in the crate's `Cargo.toml` with `{ workspace = true }`.
