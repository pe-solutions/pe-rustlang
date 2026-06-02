# Rustlang Project Euler solutions

<p align="center"><img src="logo.png"></p>

Rust solutions to [Project Euler](https://projecteuler.net/) problems, organised as a Cargo workspace. Each problem is a standalone binary crate under `solutions/pe-NNNN/`.

## Structure

```
pe-utils/          shared timing harness (pe_main! macro)
solutions/
  pe-0001/         one crate per problem
  pe-0002/
  ...
problems.toml      titles, statements, data fields, and solved flags (998 problems)
```

## Usage

```bash
# Run a solution
cargo run -p pe-0042

# Build or test all solutions
./build-all.sh
./test-all.sh

# Scaffold a new problem
./new-solution.sh 42

# Update problems.toml from projecteuler.net
./fetch-problems.sh          # recent problems only
./fetch-problems.sh --all    # full re-scrape (first-time setup)
```

## problems.toml

Each entry tracks a problem's title, statement, optional data fields, and a `solved` flag.

`data_url` — present for the ~20 problems PE hosts as a separate download (e.g. `0054_poker.txt`). `new-solution.sh` downloads it automatically into `data/NNNN_name.txt`.

`data_embedded = true` — marks problems whose input data is embedded in the problem statement (a grid, triangle, or matrix) rather than hosted as a download. The solutions for these problems (11, 18, 345) read from a `data/NNNN_*.txt` file, same convention as `data_url` problems.

## Solution pattern

Every crate exposes a `solve()` function and uses the `pe_main!()` macro to generate `main`, which times the call and prints the answer:

```rust
// Coded Triangle Numbers
// https://projecteuler.net/problem=42

fn solve() -> u64 {
    // ...
}

pe_utils::pe_main!();
```
