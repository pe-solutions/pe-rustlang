//! Mathematical and utility functions for Project Euler solutions.
//!
//! This library provides canonical implementations of common algorithms used across PE problems:
//! - **digits**: digit sum, reversal, palindrome checking, pandigital validation
//! - **primes**: Miller-Rabin and trial division primality testing
//! - **sieve**: Sieve of Eratosthenes variants (bools, primes list, omega)
//! - **modular**: modular multiplication and exponentiation
//! - **sequences**: Fibonacci iterator, polygonal numbers
//! - **divisors**: divisor sum, count, prime factorization
//! - **combinatorics**: factorial, binomial, partition counting
//! - **isqrt**: integer square root, perfect square testing
//! - **number_theory**: Euler's totient function, totient sieve
//! - **file_io**: file reading utilities (CSV, lines, strings)
//!
//! All functions are re-exported at crate root via `pub use`.

pub mod combinatorics;
pub mod digits;
pub mod divisors;
pub mod file_io;
pub mod isqrt;
pub mod modular;
pub mod number_theory;
pub mod primes;
pub mod sequences;
pub mod sieve;

pub use combinatorics::*;
pub use digits::*;
pub use divisors::*;
pub use file_io::*;
pub use isqrt::*;
pub use modular::*;
pub use number_theory::*;
pub use primes::*;
pub use sequences::*;
pub use sieve::*;
