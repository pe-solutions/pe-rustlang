//! Primality testing functions.
//!
//! Re-exported from `factorization` module for backward compatibility.
//! New code should use `factorization::is_prime` or import directly from crate root.

pub use crate::factorization::{is_prime, is_prime_trial};
