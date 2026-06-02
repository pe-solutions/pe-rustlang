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

> Solutions that read data files (e.g. `pe-0011`, `pe-0013`, `pe-0018`, `pe-0022`, `pe-0042`, `pe-0081`, `pe-0345`) use relative paths like `data/filename.txt`. Run them from the crate directory (`cd solutions/pe-0081 && cargo run`) or they will fail to open the file.

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

`data_embedded = true` marks problems whose input data is embedded directly in the problem statement (e.g. a grid or triangle of numbers) rather than hosted as a separate download. PE provides no `data_url` for these, but the data is separable from the narrative. The solutions for these problems (11, 18, 345) follow the same convention as `data_url` problems: data lives in `data/NNNN_*.txt` and is read at runtime.

`fetch-problems.sh` scrapes titles from `/archives`, statements and data URLs from `/minimal=N` (only re-fetching the ~20 problems whose statement mentions a `.txt`/`.csv` file). Re-running preserves manually set `solved` values; the flag is auto-detected for new entries (directory exists and `src/main.rs` contains no `todo!()`).

Mark a problem solved by editing `problems.toml` directly.

## Architecture

### `pe-utils` (shared library)

`pe-utils/src/lib.rs` provides two things:

- `pe_utils::run(problem_num, solve_fn)` — calls `solve_fn`, prints the problem number, answer, and elapsed milliseconds.
- `pe_utils::pe_main!()` — a macro that generates `fn main()`. It derives the problem number from the crate name at compile time (`pe-0042` → 42) and calls `pe_utils::run(42, solve)`.

### `pe-lib` (mathematical utilities library)

`pe-lib/src/` provides canonical implementations of mathematical utilities used across multiple solutions. All functions are re-exported at the crate root for convenient use.

**Available modules and functions:**

| Module | Functions | Use Cases |
|--------|-----------|-----------|
| `digits` | `digit_sum`, `digit_sum_sq`, `reverse_digits`, `is_palindrome_num`, `is_palindrome_str`, `is_pandigital`, `is_permutation`, `digits` | Digit manipulation (sums, reversals, palindromes) |
| `primes` | `is_prime`, `is_prime_trial` | Primality testing (Miller-Rabin and 6k±1 trial division) |
| `sieve` | `sieve_bools`, `sieve_primes`, `sieve_omega` | Prime sieves and factorization |
| `modular` | `mod_pow`, `mod_mul` | Modular exponentiation and multiplication |
| `sequences` | `Fibonacci`, `triangular`, `pentagonal`, `hexagonal`, etc. | Number sequences and polygonal numbers |
| `divisors` | `sum_proper_divisors`, `count_divisors`, `prime_factors`, `largest_prime_factor` | Divisor operations |
| `combinatorics` | `factorial`, `binomial_big`, `count_partitions` | Combinatorial functions |
| `isqrt` | `isqrt`, `is_perfect_square` | Integer square root operations |
| `number_theory` | `totient`, `totient_sieve` | Number theory utilities |
| `file_io` | `read_file_to_string`, `read_csv_matrix`, `read_space_separated_matrix`, `read_lines` | File I/O and parsing utilities |
| `rational` | `Rational` struct with arithmetic ops | Rational number arithmetic (fractions with automatic GCD reduction) |

**Using pe-lib in a solution:**

```rust
use pe_lib::{is_prime, digit_sum, sieve_primes};

fn solve() -> u64 {
    let primes = sieve_primes(100);
    let sum: u64 = primes.iter()
        .filter(|&p| is_prime(*p as u64))
        .map(|p| digit_sum(*p as u64))
        .sum();
    sum
}
```

**Refactoring status:** 76 of 117 solutions (64.9%) have been refactored to use `pe-lib` functions across 5 categories:
- **Tier 1** (26 solutions): Primes, digits, sieve, modular arithmetic
- **Tier 2** (10 solutions): Number theory, sequences, divisors, combinatorics, isqrt
- **Tier 3** (5 solutions): Specialized iterator patterns
- **Tier 4** (3 solutions): File I/O utilities
- **Tier 5** (1 solution): Rational Number Algebra
- **Batch 2** (18 solutions): Additional refactoring (HIGH + MEDIUM impact)
- **Batch 3** (9 solutions): Expanded range 51-100 with pe-lib functions
- **Batch 4** (4 solutions): Final 51-100 range solutions using pe-lib

Total: ~2,500 lines of duplicated code eliminated. All 117 solutions have `pe-lib` in their `Cargo.toml`.

### Testing

Comprehensive test suite with 260+ passing tests across three phases:

**Phase 1: pe-lib Unit Tests (126 tests)**
- All 13 modules tested: primes, digits, sieve, modular, sequences, divisors, combinatorics, isqrt, number_theory, file_io, rational, and more
- Edge cases, known values, mathematical properties (commutativity, associativity, overflow safety)

**Phase 2: Solution-Level Tests (117 solutions)**
- 64.9% refactored (76/117 solutions)
- Property-based testing (monotonic growth, symmetry, composition)
- Avoids hardcoding large answers; verifies correctness via properties

**Phase 3: Integration Tests (31 tests)**
- Cross-module verification: sieve/is_prime consistency, GCD/LCM relationships, prime factorization
- Cross-solution validation: pe-0010, pe-0021, pe-0066, pe-0070, pe-0076 logic
- Performance baselines: sieve <1s for 100K, is_prime scaling, GCD <1000μs
- Error boundary testing: zero/one handling, large numbers

**Test commands:**
```bash
cargo test -p pe-lib              # All pe-lib tests (157 total: 126 unit + 31 integration)
cargo test -p pe-lib --lib        # Just unit tests (126)
cargo test -p pe-lib --test integration_test  # Just integration (31)
cargo test -p pe-NNNN             # Specific solution tests
./test-all.sh                     # Full build/test (117 solutions, 100% passing)
```

**Test Results:** All 117 solutions passing (100% success rate) ✓

**Known issues fixed:**
- Miller-Rabin primality test corrected: original witness set contained values divisible by small primes (e.g., 450775 % 19 = 0), causing is_prime(19) to return false. Replaced with standard deterministic witnesses [2,3,5,7,11,13,17,19,23,29,31,37].

**Performance optimizations (commit e4d16ab):**
- `primes.rs`: Replaced `decompose()` loop with `trailing_zeros()` bit operation (1-2x faster)
- `sieve.rs`: Replaced floating-point sqrt with `isqrt()` (eliminates FP overhead)
- `divisors.rs`: Integer sqrt in `sum_proper_divisors()`, `count_divisors()`; 6k±1 pattern in `prime_factors()` and `largest_prime_factor()` (2-3x faster factorization, 66% fewer iterations)
- `digits.rs`: Zero-allocation two-pointer algorithm in `is_palindrome_str()` (3-5x faster)

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

All external crates and internal libraries are declared once in the root `Cargo.toml` under `[workspace.dependencies]` and referenced in crate manifests with `{ workspace = true }`. 

**Internal libraries:**
- `pe-utils` — timing harness and main macro
- `pe-lib` — mathematical utilities (prime checking, digit manipulation, sieve, modular arithmetic, sequences, divisors, combinatorics, etc.)

**External crates:** `num-bigint`, `num-traits`, `num`, `num-rational`, `itertools`, `primal`, `primes`, `csv`, `chrono`.

### Adding a new problem

Use the scaffolding script — it handles directory creation, boilerplate, and workspace registration in one step:

```bash
./new-solution.sh 42
```

This creates `solutions/pe-0042/` with a `Cargo.toml` and a `src/main.rs` pre-filled with the problem title (from `problems.toml`) and URL, then inserts `"solutions/pe-0042"` into the workspace `members` list in sorted order.

If the problem has a `data_url` in `problems.toml`, the scaffolding script downloads it automatically to `solutions/pe-NNNN/data/NNNN_name.txt`. Read it from `src/main.rs` using a relative path (e.g. `"data/0054_poker.txt"`). For problems without a `data_url`, place any hand-crafted data files in the same `data/` directory.

To add an external dependency, declare it once in the root `Cargo.toml` under `[workspace.dependencies]`, then reference it in the crate's `Cargo.toml` with `{ workspace = true }`.

## Refactoring to pe-lib

Solutions that define custom implementations of common algorithms can be refactored to use `pe-lib` functions, reducing code duplication and improving maintainability.

### Refactoring patterns

When refactoring a solution, replace custom function implementations with `pe-lib` imports:

**Before:**
```rust
fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 { return false; }
    }
    true
}

fn solve() -> u64 {
    (2..1_000_000).filter(|&n| is_prime(n)).sum()
}
```

**After:**
```rust
use pe_lib::is_prime;

fn solve() -> u64 {
    (2..1_000_000).filter(|&n| is_prime(n)).sum()
}
```

### Type conversions

Some solutions use integer types that differ from `pe-lib`'s canonical u64. Handle these with wrapper functions:

```rust
use pe_lib::is_prime;

fn is_prime_i32(n: i32) -> bool {
    if n <= 0 { false } else { is_prime(n as u64) }
}

fn solve() -> i32 {
    (2..=100).filter(|&n| is_prime_i32(n)).max().unwrap_or(0)
}
```

### Refactoring progress

- **45 solutions** refactored across all tiers (54.2% of 83 total)
- **38 solutions** remain (mostly domain-specific algorithms)
- **~1,814 lines** of duplicated code eliminated
- **343 tests** covering refactored code (126 unit + 31 integration + ~186 solution)

Remaining solutions are primarily domain-specific algorithms unsuitable for library extraction. See `pe-lib` module documentation for available functions.
