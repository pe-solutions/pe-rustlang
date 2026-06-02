// Even Fibonacci Numbers
// https://projecteuler.net/problem=2

use pe_lib::Fibonacci;

fn sum_of_even_fibo(limit: u64) -> u64 {
    Fibonacci::new()
        .filter(|&x| x % 2 == 0)
        .take_while(|&x| x < limit)
        .sum()
}

fn solve() -> u64 {
    sum_of_even_fibo(4_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_fibo_small() {
        let result = sum_of_even_fibo(100);
        assert_eq!(result, 44); // 2 + 8 + 34
    }

    #[test]
    fn test_even_fibo_monotonic() {
        let r1 = sum_of_even_fibo(1000);
        let r2 = sum_of_even_fibo(10000);
        assert!(r2 > r1); // More terms, larger sum
    }

    #[test]
    fn test_even_fibo_properties() {
        // All Fibonacci numbers alternately even/odd/odd pattern
        // So every third Fibonacci number is even
        let result = sum_of_even_fibo(100);
        assert!(result > 0);
    }

    #[test]
    fn test_solve_produces_result() {
        let result = solve();
        assert!(result > 0);
    }
}

pe_utils::pe_main!();
