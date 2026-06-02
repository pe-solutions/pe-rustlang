// Diophantine Equation
// https://projecteuler.net/problem=66

use num_bigint::BigInt;
use num_traits::{One, Zero};
use pe_lib::is_perfect_square;

fn find_minimal_solution(d: i32) -> (BigInt, BigInt) {
    let d_big = BigInt::from(d);
    let a0_big = BigInt::from((d as f64).sqrt() as i64);

    if &a0_big * &a0_big == d_big {
        return (BigInt::zero(), BigInt::zero());
    }

    if &a0_big * &a0_big - &d_big == BigInt::one() {
        return (a0_big, BigInt::one());
    }

    let mut m = BigInt::zero();
    let mut d_val = BigInt::one();
    let mut a = a0_big.clone();

    let mut h0 = BigInt::one();
    let mut h1 = a0_big.clone();
    let mut k0 = BigInt::zero();
    let mut k1 = BigInt::one();

    loop {
        m = &d_val * &a - &m;
        d_val = (&d_big - &m * &m) / &d_val;
        a = (&a0_big + &m) / &d_val;

        let h2 = &a * &h1 + &h0;
        let k2 = &a * &k1 + &k0;

        let lhs = &h2 * &h2;
        let rhs = &d_big * &k2 * &k2;
        
        if &lhs - &rhs == BigInt::one() {
            return (h2, k2);
        }

        h0 = h1;
        h1 = h2;
        k0 = k1;
        k1 = k2;
    }
}

fn solve() -> i32 {
    let mut max_x = BigInt::zero();
    let mut max_d = 0;
    for d in 2..=1000 {
        if is_perfect_square(d as u64) { continue; }
        let (x, _y) = find_minimal_solution(d);
        if x > max_x { max_x = x; max_d = d; }
    }
    max_d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_square_skipping() {
        // Verify we skip perfect squares
        assert!(is_perfect_square(4));
        assert!(is_perfect_square(9));
        assert!(is_perfect_square(16));
        assert!(!is_perfect_square(5));
        assert!(!is_perfect_square(10));
    }

    #[test]
    fn test_find_minimal_solution_simple() {
        // For d=2, find x,y where x^2 - 2*y^2 = 1
        let (x, y) = find_minimal_solution(2);
        // Verify the solution satisfies the equation: x^2 - 2*y^2 = 1
        let lhs = &x * &x - 2 * &y * &y;
        assert_eq!(lhs, BigInt::one());
    }

    #[test]
    fn test_find_minimal_solution_perfect_square() {
        // For d=4 (perfect square), should return (0,0)
        let (x, y) = find_minimal_solution(4);
        assert_eq!(x, BigInt::zero());
        assert_eq!(y, BigInt::zero());
    }

    #[test]
    fn test_solve_produces_output() {
        let result = solve();
        assert!(result > 0);
        assert!(result <= 1000);
    }
}

pe_utils::pe_main!();
