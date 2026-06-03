//! Mathematical and utility functions for Project Euler solutions.
//!
//! This library provides canonical implementations of common algorithms used across PE problems:
//! - **factorization**: primality testing and integer factorization
//! - **digits**: digit sum, reversal, palindrome checking, pandigital validation
//! - **sieve**: Sieve of Eratosthenes variants (bools, primes list, omega)
//! - **modular**: modular multiplication and exponentiation
//! - **sequences**: Fibonacci iterator, polygonal numbers
//! - **divisors**: divisor sum, count
//! - **combinatorics**: factorial, binomial, partition counting
//! - **isqrt**: integer square root, perfect square testing
//! - **number_theory**: Euler's totient function, totient sieve
//! - **generic_int**: generic integer operations (u64, u128, extensible)
//! - **rational**: rational number arithmetic (addition, subtraction, multiplication, division)
//!
//! File I/O utilities are available via pe-utils (which pe-lib re-exports for convenience).
//! All functions are re-exported at crate root via `pub use`.

pub mod combinatorics;
pub mod digits;
pub mod divisors;
pub mod factorization;
pub mod generic_int;
pub mod isqrt;
pub mod modular;
pub mod number_theory;
pub mod primes;
pub mod rational;
pub mod sequences;
pub mod sieve;

pub use combinatorics::*;
pub use digits::*;
pub use divisors::*;
pub use factorization::*;
pub use generic_int::*;
pub use isqrt::*;
pub use modular::*;
pub use number_theory::*;
pub use rational::*;
pub use sequences::*;
pub use sieve::*;
pub use pe_utils::file_io::*;
