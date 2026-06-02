// Two-Dimensional Recurrence
// https://projecteuler.net/problem=940

use pe_lib::Fibonacci;

const MOD: i64 = 1_123_581_313;
const MAX_K: usize = 50;

/// Computes the nth term of a linear recurrence:
/// f[n] = a * f[n-1] + b * f[n-2], given f(0) and f(1)
fn solve_recurrence(initial0: i64, initial1: i64, a: i64, b: i64, mut n: u64) -> i64 {
    let mut coeff0 = 1;
    let mut coeff1 = 0;
    let (mut base0, mut base1) = (0, 1);

    while n > 0 {
        if n % 2 == 1 {
            let new_coeff0 = (coeff0 * base0 + b * coeff1 * base1) % MOD;
            let new_coeff1 = (coeff0 * base1 + coeff1 * base0 + a * coeff1 * base1) % MOD;
            coeff0 = new_coeff0;
            coeff1 = new_coeff1;
        }

        let new_base0 = (base0 * base0 + b * base1 * base1) % MOD;
        let new_base1 = (2 * base0 * base1 + a * base1 * base1) % MOD;
        
        base0 = new_base0;
        base1 = new_base1;

        n /= 2;
    }

    (initial0 * coeff0 + initial1 * coeff1) % MOD
}

/// Computes A(n, m) based on nested linear recurrences
fn compute_a(n: u64, m: u64) -> i64 {
    let f0 = solve_recurrence(0, 1, 3, 1, n);
    let f1 = solve_recurrence(1, 2, 3, 1, n);
    
    solve_recurrence(f0, f1, 1, 3, m)
}

fn solve() -> i64 {
    let fib_values: Vec<u64> = Fibonacci::new().take(MAX_K + 1).collect();
    let mut total = 0i64;
    for i in 2..=MAX_K {
        let fi = fib_values[i];
        for j in 2..=MAX_K {
            let fj = fib_values[j];
            total = (total + compute_a(fi, fj)) % MOD as i64;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_iterator() {
        let fib: Vec<u64> = Fibonacci::new().take(10).collect();
        assert_eq!(fib[0], 0);
        assert_eq!(fib[1], 1);
        assert_eq!(fib[2], 1);
        assert_eq!(fib[3], 2);
    }

    #[test]
    fn test_modulo_property() {
        let val = 1_000_000_000i64;
        let result = val % MOD;
        assert!(result < MOD);
    }

    #[test]
    fn test_solve_recurrence_simple() {
        // Test with simple recurrence
        let result = solve_recurrence(0, 1, 3, 1, 0);
        assert!(result >= 0);
    }

    #[test]
    fn test_compute_a_basic() {
        // Test that compute_a returns valid result
        let result = compute_a(1, 1);
        assert!(result < MOD);
    }

    #[test]
    fn test_solve_produces_result() {
        let result = solve();
        assert!(result >= 0);
        assert!(result < MOD);
    }
}

pe_utils::pe_main!();
