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
problems.toml      problem titles, statements, and solved flags (998 problems)
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
