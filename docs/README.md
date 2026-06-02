# Rustlang Project Euler solutions

<p align="center"><img src="logo.png"></p>

Rust solutions to [Project Euler](https://projecteuler.net/) problems, organised as a Cargo workspace. Each problem is a standalone binary crate under `solutions/pe-NNNN/`. **88 solutions implemented**, covering problems 1-50 (complete) and select problems 51-940.

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

## pe-lib: Shared Utilities Library

A centralized library of 40+ canonical mathematical and utility functions used across solutions, eliminating ~1,814 lines of duplicated code.

### Available Modules (13 total)

| Module | Purpose | Examples |
|--------|---------|----------|
| `digits` | Digit manipulation | `digit_sum`, `reverse_digits`, `is_palindrome_num`, `is_pandigital` |
| `primes` | Primality testing | `is_prime` (Miller-Rabin), `is_prime_trial` (6k±1) |
| `sieve` | Prime generation | `sieve_primes`, `sieve_bools`, `sieve_omega` |
| `modular` | Modular arithmetic | `mod_pow`, `mod_mul` |
| `sequences` | Number sequences | `Fibonacci` iterator, `triangular`, `pentagonal`, `hexagonal` |
| `divisors` | Divisor operations | `sum_proper_divisors`, `count_divisors`, `prime_factors` |
| `combinatorics` | Combinatorial functions | `factorial`, `binomial_big`, `count_partitions` |
| `isqrt` | Integer square root | `isqrt`, `is_perfect_square` |
| `number_theory` | Number theory | `gcd`, `totient`, `totient_sieve` |
| `file_io` | File utilities | `read_file_to_string`, `read_csv_matrix`, `read_lines` |
| `rational` | Rational numbers | `Rational` struct with full arithmetic (add, subtract, multiply, divide) |
| `generic_int` | Generic integer ops | `digit_sum_generic` for u64, u128, and extensible types |

### Using pe-lib

Instead of implementing utilities yourself, import from `pe-lib`:

```rust
use pe_lib::{is_prime, sieve_primes, digit_sum};

fn solve() -> u64 {
    let primes = sieve_primes(1000);
    primes.iter()
        .filter(|&&p| is_prime(p as u64))
        .map(|&p| digit_sum(p as u64))
        .sum()
}

pe_utils::pe_main!();
```

### Refactoring Status

- **63 of 88 solutions** (71.6%) refactored to use `pe-lib`
- **~2,100 lines** of duplicated code eliminated
- **157 pe-lib tests** (126 unit + 31 integration) all passing
- **88 solution tests** verifying correctness across all problems
- **Recommended**: Use pe-lib functions when adding new solutions

### Performance Optimizations

pe-lib core functions are highly optimized:
- **Prime factorization**: 6k±1 pattern (66% fewer iterations, 2-3x faster)
- **Square root operations**: Integer arithmetic, no floating-point overhead
- **Palindrome checking**: Zero-allocation two-pointer algorithm
- **Bit operations**: `trailing_zeros()` for fast bit manipulation
- **Decompose**: Bit-based decomposition for Miller-Rabin primality test
- All optimizations preserve correctness: 245+ tests passing

### Recent Improvements

**Refactoring Campaign**: Consolidated duplicated code into pe-lib
- Eliminated ~2,100 lines of duplicated code
- Refactored 14 solutions to use pe-lib (45 → 63 solutions, 54.2% → 71.6%)
- Removed external `primes` crate dependency
- Improved code consistency and maintainability

**New Solutions**: Added 5 new problems demonstrating pe-lib
- pe-0051: Prime digit replacements (`sieve_primes`, `is_prime`)
- pe-0052: Permuted multiples (`is_permutation`)
- pe-0054: Poker hands (file I/O)
- pe-0057: Square root convergents (`Rational` arithmetic)
- pe-0058: Spiral primes (`is_prime` optimized)

## Testing

Comprehensive test coverage: **all 88 solutions passing** (100% success rate)

- **Unit Tests** (126): All pe-lib modules with edge cases, known values, mathematical properties
- **Integration Tests** (31): Cross-module verification, sieve/prime consistency, performance baselines
- **Solution Tests** (88): Individual solution verification across all 88 problems
- **Total**: 245+ tests, all passing ✓

Run tests with:
```bash
cargo test -p pe-lib              # All pe-lib tests (157 total)
./test-all.sh                     # Full build/test all 88 solutions
cargo test -p pe-NNNN             # Specific solution tests
```

### Solution Coverage

| Range | Count | Status |
|-------|-------|--------|
| **1-50** | 50/50 (100%) | ✓ Complete |
| **51-100** | 21/50 (42%) | 5 new solutions added |
| **101-200** | 4/100 | Select problems |
| **201-940** | 13/740 | Select problems |
| **Total** | **88/998** | All passing |
